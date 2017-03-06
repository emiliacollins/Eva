//##################################################################################################
//#                                                                                                #
//# Kernel/memory/paging mod.rs                                                                    #
//#                                                                                                #
//# AUTHOR: Eric S. Collins <ericscollins@protonmail.com>                                          #
//#                                                                                                #
//#                                                                                                #
//# MIT LICENSE                                                                                    #
//# ---------------------------------------------------------------------------------------------- #
//#                                                                                                #
//# Copyright 2017 Eric S. Collins                                                                 #
//#                                                                                                #
//# Permission is hereby granted, free of charge, to any person obtaining a copy of this software  #
//# and associated documentation files (the "Software"), to deal in the Software without           #
//# restriction, including without limitation the rights to use, copy, modify, merge, publish,     #
//# distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the  #
//# Software is furnished to do so, subject to the following conditions:                           #
//#                                                                                                #
//# The above copyright notice and this permission notice shall be included in all copies or       #
//# substantial portions of the Software.                                                          #
//#                                                                                                #
//# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING  #
//# BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND     #
//# NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,   #
//# DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, #
//# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.        #
//#                                                                                                #
//# ---------------------------------------------------------------------------------------------- #
//#                                                                                                #
//#                                                                                                #
//# NOTE:   The majority of code in this file was written while closely following a tutorial       #
//#         createad by Philip Opperman <contact@phil-opp.com>. The tutorial may be found at:      #
//#                                    http://os.phil-opp.com/                                     #
//#         Source code used the above tutorial may be found at:                                   #
//#                             https://github.com/phil-opp/blog_os                                #
//#                                                                                                #
//##################################################################################################


//##################################################################################################
//**************************************** DEPENDENCIES ********************************************
//##################################################################################################

use memory::PAGE_SIZE;
use memory::{FrameAllocator,Frame};
use self::entry::{EntryFlags,HUGE_PAGE,PRESENT};
use memory::paging::table::PAGE_MAP;
use self::table::{Table,PageMap};
use core::ptr::Unique;

extern crate x86;

//##################################################################################################
//***************************************** CONSTANTS **********************************************
//##################################################################################################


const ENTRY_COUNT: usize = 512;


//##################################################################################################
//***************************************** SUBMODULES *********************************************
//##################################################################################################


mod entry;
mod table;


//##################################################################################################
//********************************** TYPE & STRUCT DEFINITIONS *************************************
//##################################################################################################


//==================================================================================================
pub type PhysicalAddress = usize;
//--------------------------------------------------------------------------------------------------
// Alias for usize to specify that a given address represents a physical address.
//==================================================================================================


//==================================================================================================
type VirtualAddress = usize;
//--------------------------------------------------------------------------------------------------
// Alias for usize to specify that a given address represents a virtual address.
//==================================================================================================


//==================================================================================================
pub struct ActivePageTable {
//--------------------------------------------------------------------------------------------------
// Struct to provide unique ownership of page map.
//==================================================================================================

    page_map: Unique<Table<PageMap>>,   
}


//==================================================================================================
pub struct Page {
//--------------------------------------------------------------------------------------------------
// Object representing a memory page.    
//==================================================================================================
    
    page_num: usize,                    // Page index
}


//##################################################################################################
//************************************* STRUCT IMPLEMENTATIONS *************************************
//##################################################################################################


//==================================================================================================
impl ActivePageTable {
//==================================================================================================


    //==============================================================================================
    pub unsafe fn new() -> ActivePageTable {
    //----------------------------------------------------------------------------------------------
    // Pseudo-constructor for ActivePageTable.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: An ActivePageTable wrapping a unique pointer to the page map
    //==============================================================================================
    
        ActivePageTable {
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

        let mut page_table = self.page_map_mut().next_table_mut(page.page_map_index())
            .and_then(|ptr_tbl| ptr_tbl.next_table_mut(page.pointer_table_index()))
            .and_then(|pg_dir| pg_dir.next_table_mut(page.page_dir_index()))
            .expect("Huge pages not currently supported!");

        let frame = page_table[page.page_table_index()].target_frame().expect("Page not mapped!");
        page_table[page.page_table_index()].mark_unused();
        unsafe {x86::shared::tlb::flush(page.starting_address());}
        //allocator.deallocate_frame(frame);
  
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
            println!("Reached invalid next table");
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

        println!("Attempting resolve");
        // Attempt to resolve virtual
        pointer_table.and_then(|ptr_tbl| ptr_tbl.next_table(page.pointer_table_index()))
            .and_then(|pg_dir| pg_dir.next_table(page.page_dir_index()))
            .and_then(|pg_tbl| pg_tbl[page.page_table_index()].target_frame())
            .or_else(check_huge_page)
    } 
}


//==================================================================================================
impl Page {
//==================================================================================================


    //==============================================================================================
    pub fn starting_address(&self) -> PhysicalAddress {
    //----------------------------------------------------------------------------------------------
    // Obtain the starting address of the page.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: Physical starting address of the page
    //==============================================================================================

        self.page_num * PAGE_SIZE
    }
    

    //==============================================================================================
    pub fn containing_address(addr: VirtualAddress) -> Page {
    //----------------------------------------------------------------------------------------------
    // Obtain a page containing the given virtual address.
    //----------------------------------------------------------------------------------------------
    // TAKES:   addr -> VirtualAddress contained within desired page
    //
    // RETURNS: page containing given virtual address
    //==============================================================================================

        Page { page_num: addr / PAGE_SIZE }
    }

    //==============================================================================================
    pub fn page_map_index(&self) -> usize {
    //----------------------------------------------------------------------------------------------
    // Extracts the page map index from the page's starting address.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: index in the page map
    //==============================================================================================

       self.page_num  >> 27 & 0x1FF
    }

    //==============================================================================================
    pub fn pointer_table_index(&self) -> usize {
    //----------------------------------------------------------------------------------------------
    // Extracts the pointer table index from a virtual address.
    //----------------------------------------------------------------------------------------------
    // TAKES:   i -> virtual address to extract from
    //
    // RETURNS: index in the pointer table
    //==============================================================================================

        self.page_num >> 18 & 0x1FF
    }

    
    //==============================================================================================
    pub fn page_dir_index(&self) -> usize {
    //----------------------------------------------------------------------------------------------
    // Extracts the page directory index from a virtual address.
    //----------------------------------------------------------------------------------------------
    // TAKES: i -> virtual address to extract from
    //
    // RETURNS: index in the page directory
    //==============================================================================================

        self.page_num >> 9 & 0x1FF
    }

    
    //==============================================================================================
    pub fn page_table_index(&self) -> usize {
    //----------------------------------------------------------------------------------------------
    // Extracts the page table index from a virtual address.
    //----------------------------------------------------------------------------------------------
    // TAKES:   i -> virtual address to extract from
    //
    // RETURNS: index in the page table
    //==============================================================================================

        self.page_num >> 0 & 0x1FF
    }
}


//##################################################################################################
//************************************************* TESTS ******************************************
//##################################################################################################


pub fn test_paging<A: FrameAllocator>(allocator: &mut A) {

    let mut page_map = unsafe {ActivePageTable::new()};

    println!("Some = {:?}", page_map.translate(0));
    println!("Some = {:?}", page_map.translate(4096)); // second P1 entry
    println!("Some = {:?}", page_map.translate(512 * 4096)); // second P2 entry
    println!("Some = {:?}", page_map.translate(300 * 512 * 4096)); // 300th P2 entry
    
    println!("None = {:?}", page_map.translate(512 * 512 * 4096)); // second P3 entry



    
    page_map.translate(0);
    page_map.translate(4096);
    page_map.translate(512 * 4096);
    page_map.translate(300 * 512 * 4096);
    page_map.translate(0);
    page_map.translate(4096);
    page_map.translate(512 * 4096);
    page_map.translate(300 * 512 * 4096);
       
    
    println!("Some = {:?}", page_map.translate(512 * 512 * 4096 - 1)); // last mapped byte
    


    let addr = 42 * 512 *512 * 4096;
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}",
             page_map.translate(addr),
             frame);
    page_map.map_page_to_frame(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_map.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());



    page_map.unmap(Page::containing_address(addr), allocator);
    println!("None={:?}", page_map.translate(addr));

    println!("{:#x}", unsafe {
    *(Page::containing_address(addr).starting_address() as *const u64)
    });
    
}
