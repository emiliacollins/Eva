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
//************************************ CRATES & SUBMODULES *****************************************
//##################################################################################################


mod entry;
mod table;
mod temp_page;
mod pt_mapper;


//##################################################################################################
//**************************************** DEPENDENCIES ********************************************
//##################################################################################################


use memory::{FrameAllocator,Frame,PAGE_SIZE};
use self::entry::{EntryFlags,HUGE_PAGE,PRESENT,WRITABLE};
use memory::paging::table::PAGE_MAP;
use self::table::{Table,PageMap};
use core::ptr::Unique;
use core::ops::{Deref,DerefMut};
use memory::paging::temp_page::TempPage;
use self::pt_mapper::PTMapper;
use ::x86::shared::{control_regs,tlb};
use multiboot2::BootInformation;
    


//##################################################################################################
//***************************************** CONSTANTS **********************************************
//##################################################################################################


const ENTRY_COUNT: usize = 512;
const MAGIC_PAGE_NUMBER: usize = 0xDEADBEEF;


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
// Page table currently loaded into the C3 register.
//==================================================================================================

    mapper: PTMapper,
}


//==================================================================================================
pub struct InactivePageTable {
//--------------------------------------------------------------------------------------------------
// Page table not currently loaded into the C3 register.
//==================================================================================================

    page_map_frame: Frame,
}


//==================================================================================================
#[derive(Debug, Copy, Clone)]
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

    unsafe fn new() -> ActivePageTable {
        ActivePageTable { mapper: PTMapper::new(), }
    }

    
    //==============================================================================================
    pub fn with<L: FnOnce(&mut PTMapper)>(&mut self, table: &mut InactivePageTable, temp_page: &mut TempPage,
                                          lambda: L) {
    //----------------------------------------------------------------------------------------------
    // Run a given closure on an inactive page table.
    //----------------------------------------------------------------------------------------------
    // TAKES:   table     -> table to run closure on
    //          temp_page -> page to store active page table to restore recursive entry    
    //          lambda    -> closure to run on table
    //
    // RETURNS: nothing
    //==============================================================================================


        let orig_ctrl3 = Frame::frame_containing_address(unsafe { control_regs::cr3() } as usize);

        {
            let active_table = temp_page.map_to_frame_as_table(orig_ctrl3.clone(), self);
            
            self.page_map_mut()[ENTRY_COUNT-1].set(table.page_map_frame.clone(), PRESENT | WRITABLE);
            unsafe { tlb::flush_all() };

            lambda(self);

            active_table[ENTRY_COUNT - 1].set(orig_ctrl3, PRESENT | WRITABLE);
            unsafe { tlb::flush_all() };
        }
        
        temp_page.unmap(self);
    }


    pub fn switch(&mut self, inactive_table: InactivePageTable) -> InactivePageTable {
        let orig_table = InactivePageTable
        {
            page_map_frame: Frame::frame_containing_address(unsafe { control_regs::cr3() } as usize),
        };

        unsafe { control_regs::cr3_write(inactive_table.page_map_frame.address()); }

        orig_table
    }

}


//==================================================================================================
impl Deref for ActivePageTable {
//==================================================================================================

    type Target = PTMapper;

    fn deref(&self) -> &PTMapper {
        &self.mapper
    }
}


//==================================================================================================
impl DerefMut for ActivePageTable {
//==================================================================================================

    fn deref_mut(&mut self) -> &mut PTMapper {
        &mut self.mapper
    }
}


//==================================================================================================



//==================================================================================================
impl InactivePageTable {
//==================================================================================================


    //==============================================================================================
    pub fn new(frame: Frame, active_table: &mut ActivePageTable, temp_page: &mut TempPage) -> InactivePageTable {
    //----------------------------------------------------------------------------------------------
    // Pseudoconstructor for InactivePageTable.
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame        -> frame to map the InactivePageTable's PageMap to
    //          active_table -> page table currently in use
    //          temp_page    -> temporary page used to zero out inactive PageMap
    //    
    // RETURNS: an instance of InactivePageTable mapped to `frame` in `Active
    //==============================================================================================

        {
            let table = temp_page.map_to_frame_as_table(frame.clone(), active_table);
            table.clear();
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }

        temp_page.unmap(active_table);
        
        InactivePageTable { page_map_frame: frame }
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
//***************************************** PUBLIC FUNCTIONS ***************************************
//##################################################################################################


//==================================================================================================
pub fn remap_kernel<F: FrameAllocator>(allocator: &mut F, boot_info: &BootInformation) {
//--------------------------------------------------------------------------------------------------
//
//--------------------------------------------------------------------------------------------------
//
//
//
//==================================================================================================

    
    let mut temp_page = TempPage::new(Page { page_num: MAGIC_PAGE_NUMBER }, allocator);

    let mut active_table = unsafe { ActivePageTable::new() };

    let mut inactive_table = InactivePageTable::new(allocator.allocate_frame().expect("no frames!"),
                                                    &mut active_table, &mut temp_page);


    active_table.with(&mut inactive_table, &mut temp_page, |pt_mapper| {

        // Identity map the kernel
        for section in boot_info.elf_sections_tag().expect("multiboot tag required!").sections() {
            if (!section.is_allocated()) { continue; }
            println!("mapping sect w/ addr={:#x} & size={:#x}", section.addr, section.size);

            let flags = EntryFlags::from_elf_section(section);
            assert!(section.addr as usize % PAGE_SIZE == 0,
                    "sections need to be page aligned");
            for frame_num in Frame::frame_containing_address(section.start_address()).frame_num ..
                Frame::frame_containing_address(section.end_address()).frame_num {
                    pt_mapper.identity_map(Frame { frame_num: frame_num }, flags, allocator);
                }
        }

        // Identity map the VGA buffer
        pt_mapper.identity_map(Frame::frame_containing_address(::vga_interface::VGA_BUFFER_START as usize),
                               WRITABLE, allocator);

        // Identity map the boot information structure
        for frame_num in Frame::frame_containing_address(boot_info.start_address()).frame_num..
            Frame::frame_containing_address(boot_info.end_address()).frame_num {
                pt_mapper.identity_map(Frame { frame_num: frame_num }, PRESENT, allocator);
            }
    });
    
    let orig_table = active_table.switch(inactive_table);
    println!("SWITCHED");

    active_table.unmap(Page::containing_address(orig_table.page_map_frame.address()), allocator);

    println!("guard page active!");
}


