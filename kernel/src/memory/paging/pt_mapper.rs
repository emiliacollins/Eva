
use core::ptr::Unique;
use memory::paging::table::{PageMap,Table,PAGE_MAP};
use memory::paging::entry::{EntryFlags,PRESENT,HUGE_PAGE};
use memory::paging::{Page,VirtualAddress,PhysicalAddress,InactivePageTable,ENTRY_COUNT};
use memory::{Frame,FrameAllocator,PAGE_SIZE};
use ::x86::shared::tlb;

//==================================================================================================
pub struct PTMapper {
//--------------------------------------------------------------------------------------------------
// Struct to provide unique ownership of page map.
//==================================================================================================

    page_map: Unique<Table<PageMap>>,   
}

//==================================================================================================
impl PTMapper {
//==================================================================================================


    //==============================================================================================
    pub unsafe fn new() -> PTMapper {
    //----------------------------------------------------------------------------------------------
    // Pseudo-constructor for Mapper.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: An Mapper wrapping a unique pointer to a page map
    //==============================================================================================
    
        PTMapper {
            page_map: Unique::new(PAGE_MAP),
        }
    }

    
    //==============================================================================================
    pub fn page_map(&self) -> &Table<PageMap> {
    //----------------------------------------------------------------------------------------------
    // Obtain an immutable reference to the page map.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: An immutable reference to the page map
    //==============================================================================================

        unsafe { self.page_map.get() }
    }


    //==============================================================================================
    pub fn page_map_mut(&mut self) -> &mut Table<PageMap> {
    //----------------------------------------------------------------------------------------------
    // Obtain a mutable reference to the page map.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: A mutable reference to the page map
    //==============================================================================================

        unsafe { self.page_map.get_mut() }
    }

    
    //==================================================================================================
    pub fn translate(&self, v_addr: VirtualAddress) -> Option<PhysicalAddress> {
    //--------------------------------------------------------------------------------------------------
    // Translate a virtual address to its corresponding physical address, if possible.
    //--------------------------------------------------------------------------------------------------
    // TAKES:   v_addr -> virtual address to translate
    //
    // RETURNS: Some(...) -> physical address corresponding to virtual address
    //          None      -> virtual address was invalid   
    //==================================================================================================

        let page_offset = v_addr & 0x0FFF;
        self.translate_page_to_frame(Page::containing_address(v_addr))
            .map(|frame| frame.frame_num * PAGE_SIZE + page_offset)
    }


    //==================================================================================================
    pub fn map_page<A: FrameAllocator>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A) {
    //--------------------------------------------------------------------------------------------------
    // Map the given page to the next free frame.
    //--------------------------------------------------------------------------------------------------
    // TAKES:   page      -> page to be mapped
    //          flags     -> flags to set the entry with
    //          allocator -> allocator to allocate frame to be mapped to
    //
    // RETURNS: nothing
    //==================================================================================================

        self.map_page_to_frame(page, allocator.allocate_frame().expect("unable to allocate frame"),
                               flags, allocator);
        
    }


    //==================================================================================================
    pub fn map_page_to_frame<A: FrameAllocator>(&mut self, page: Page, frame: Frame,
                                                flags: EntryFlags, allocator: &mut A) {
    //--------------------------------------------------------------------------------------------------
    // Map the given page to the given frame.
    //--------------------------------------------------------------------------------------------------
    // TAKES:   page      ->
    //          frame     -> 
    //          flags     ->
    //          allocator ->
    //
    // RETURNS: nothing
    //==================================================================================================        

        self.page_map_mut().next_table_create(page.page_map_index(), allocator)
            .next_table_create(page.pointer_table_index(), allocator)
            .next_table_create(page.page_dir_index(), allocator)
            [page.page_table_index()].set(frame, flags | PRESENT);
    }


    //==================================================================================================
    pub fn identity_map<A: FrameAllocator>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A) {
    //--------------------------------------------------------------------------------------------------
    // Identity map the given frame.
    //--------------------------------------------------------------------------------------------------
    // TAKES:   frame     -> frame to identity map to a page 
    //          flags     -> flags to set in identity mapped entry
    //          allocator -> allocator to allocate new tables if necessary
    //
    // RETURNS: nothing
    //==================================================================================================
        
        self.map_page_to_frame(Page::containing_address(frame.address()), frame, flags, allocator);
    }


    //==============================================================================================
    pub fn unmap<A: FrameAllocator>(&mut self, page: Page, allocator: &mut A) {
    //--------------------------------------------------------------------------------------------------
    // Unmap a given page.
    //--------------------------------------------------------------------------------------------------
    // TAKES:   page      -> page to unmap
    //          allocator -> allocator to allocate new tables if necessary
    //
    // RETURNS: nothing
    //==================================================================================================

        assert!(self.translate(page.starting_address()).is_some());
        
        let mut page_table = self.page_map_mut().next_table_mut(page.page_map_index())
            .and_then(|ptr_tbl| ptr_tbl.next_table_mut(page.pointer_table_index()))
            .and_then(|pg_dir| pg_dir.next_table_mut(page.page_dir_index()))
            .expect("Huge pages not currently supported!");

        let frame = page_table[page.page_table_index()].target_frame().expect("Page not mapped!");
        page_table[page.page_table_index()].mark_unused();
        unsafe {tlb::flush(page.starting_address());}
    }

    
    //==================================================================================================
    fn translate_page_to_frame(&self, page: Page) -> Option<Frame> {
    //--------------------------------------------------------------------------------------------------
    // Attempt to obtain the frame in which a page is loaded.
    //--------------------------------------------------------------------------------------------------
    // TAKES:   page -> page to locate containing frame for
    //
    // RETURNS: Some(...) -> Frame in which the page is loaded
    //          None      -> Page is not currently loaded in a frame    
    //==================================================================================================


        let pointer_table = self.page_map().next_table(page.page_map_index());

        // Lambda to determine if table lookup failure was due to huge pages
        let check_huge_page = |   | {
            pointer_table.and_then(|ptr_tbl| {
                // Get the appropriate entry in the pointer table
                let ptr_tbl_entry = &ptr_tbl[page.pointer_table_index()];
                // Ensure entry is present
                if let Some(frame) = ptr_tbl_entry.target_frame() {
                    // Check if entry contains huge page
                    if (ptr_tbl_entry.flags().contains(HUGE_PAGE)) {
                        // Return an adjusted frame
                        return Some(
                            Frame {
                                frame_num: frame.frame_num
                                    + ENTRY_COUNT * page.page_dir_index()
                                    + page.page_table_index(),
                            }
                        );
                    }
                }
                // Check if no huge page in pointer table, check if page directory is present
                if let Some(pg_dir) = ptr_tbl.next_table(page.pointer_table_index()) {
                    
                    // Page directory present, get entry
                    let pg_dir_entry = &pg_dir[page.page_dir_index()];
                    // Check if entry is present
                    if let Some(frame) = pg_dir_entry.target_frame() {
                        // Entry present, check if huge
                        if (pg_dir_entry.flags().contains(HUGE_PAGE)) {
                            // Entry huge, return adjusted frame
                            return Some(
                                Frame {
                                    frame_num: frame.frame_num + page.page_table_index(),
                                }
                            );
                        }
                    }
                }
                // No huge entries,or entry was not present
                None
            }
            )
        };

        // Attempt to resolve virtual
        pointer_table.and_then(|ptr_tbl| ptr_tbl.next_table(page.pointer_table_index()))
            .and_then(|pg_dir| pg_dir.next_table(page.page_dir_index()))
            .and_then(|pg_tbl| pg_tbl[page.page_table_index()].target_frame())
            .or_else(check_huge_page)
    } 
}
