//##################################################################################################
//#                                                                                                #
//# Kernel/memory/paging temp_page.rs                                                              #
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
//#         created by Philip Opperman <contact@phil-opp.com>. The tutorial may be found at:       #
//#                                    http://os.phil-opp.com/                                     #
//#         Source code used in the above tutorial may be found at:                                #
//#                             https://github.com/phil-opp/blog_os                                #
//#                                                                                                #
//##################################################################################################


//##################################################################################################
//**************************************** DEPENDENCIES ********************************************
//##################################################################################################


use super::{Page,ActivePageTable,VirtualAddress};
use super::entry::WRITABLE;
use memory::{Frame,FrameAllocator};
use super::table::{Table,PageTable};


//##################################################################################################
//******************************************** CONSTANTS *******************************************
//##################################################################################################


const MINI_FRAME_COUNT: usize = 3;


//##################################################################################################
//************************************** STRUCT DEFINITIONS ****************************************
//##################################################################################################


//==================================================================================================
pub struct TempPage {
//--------------------------------------------------------------------------------------------------
// A page easily mapped and unmapped for temporary, nonstandard use of the paging module.
//==================================================================================================
    
    page: Page,
    allocator: MiniAllocator,
}


//==================================================================================================
struct MiniAllocator([Option<Frame>; MINI_FRAME_COUNT]);
//--------------------------------------------------------------------------------------------------
// An allocator holding exactly three frames, used when manipulating inactive page tables.
//==================================================================================================


//##################################################################################################
//************************************ STRUCT IMPLEMENTATIONS **************************************
//##################################################################################################


//==================================================================================================
impl TempPage {
//==================================================================================================
    

    //==============================================================================================
    pub fn new<F:FrameAllocator>(page: Page, allocator: &mut F) -> TempPage {
    //----------------------------------------------------------------------------------------------
    // Pseudoconstructor for TempPage. Obtains necessary frames for mapping from given allocator at
    // construction.    
    //----------------------------------------------------------------------------------------------
    // TAKES:   page      -> page to map TempPage to
    //          allocator -> allocator to obtain frames from necessary for mapping
    //
    // RETURNS: TempPage constructed with given parameters
    //==============================================================================================

        TempPage {
            page: page,
            allocator: MiniAllocator::new(allocator),
        }
    }

    
    //==============================================================================================
    pub fn map_to_frame(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
    //----------------------------------------------------------------------------------------------
    // Map this temporary page to the given frame in the given active page table.
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame        -> frame to map page to
    //          active_table -> table to map frame in
    //
    // RETURNS: Virtual address of the temporary page.
    //==============================================================================================

        active_table.map_page_to_frame(self.page, frame, WRITABLE, &mut self.allocator);
        self.page.starting_address()
    }


    //==============================================================================================
    pub fn map_to_frame_as_table(&mut self, frame: Frame, active_table: &mut ActivePageTable)
                                 -> &mut Table<PageTable> {
    //----------------------------------------------------------------------------------------------
    // Perform same action as map_to_frame, but return VirtualAddress casted as a PageTable.
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame        -> frame to map page to
    //          active_table -> table to map frame in
    //
    // RETURNS: a mutable reference to a PageTable
    //==============================================================================================

        unsafe { &mut *(self.map_to_frame(frame, active_table) as *mut Table<PageTable>) }
    }  

    
    //==============================================================================================
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
    //----------------------------------------------------------------------------------------------
    // Unmap this page from the given page table.
    //----------------------------------------------------------------------------------------------
    // TAKES:   active_table -> table to unmap this page from
    //
    // RETURNS: nothing
    //==============================================================================================

        active_table.unmap(self.page, &mut self.allocator);
    }
}


//==================================================================================================
impl MiniAllocator {
//==================================================================================================


    //==============================================================================================
    fn new<A: FrameAllocator>(allocator: &mut A) -> MiniAllocator {
    //----------------------------------------------------------------------------------------------
    // Pseudo constructor for MiniAllocator, obtaining MINI_FRAME_COUNT frames from the given
    // allocator and storing them in an internal list.
    //----------------------------------------------------------------------------------------------
    // TAKES:   allocator -> FrameAllocator to obtain frames from
    //
    // RETURNS: MiniAllocator constructed from given parameters
    //==============================================================================================

        MiniAllocator([allocator.allocate_frame(), allocator.allocate_frame(), allocator.allocate_frame()])
    }


}


//==================================================================================================
impl FrameAllocator for MiniAllocator {
//==================================================================================================


    //==============================================================================================
    fn allocate_frame(&mut self) -> Option<Frame> {
    //----------------------------------------------------------------------------------------------
    // Allocate the first available frame from internal list. Removes frame from list on allocation.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: Some(...) -> allocated frame
    //          None      -> no frames available to allocate    
    //==============================================================================================

        for frame_opt in &mut self.0 {
            if frame_opt.is_some() {
                return frame_opt.take();
            }
        }   
        None
    }

    
    //==============================================================================================
    fn deallocate_frame(&mut self, frame: Frame) {
    //----------------------------------------------------------------------------------------------
    // Deallocate given frame, placing it back in the internal list. Should only deallocate frames
    // that were allocated by this instance of MiniAllocator.    
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame -> frame to deallocate
    //
    // RETURNS: nothing
    //==============================================================================================

        for frame_opt in &mut self.0 {
            if frame_opt.is_none() {
                *frame_opt = Some(frame);
                return;
            }
        }
    }

    
}
