//##################################################################################################
//#                                                                                                #
//# Kernel/memory: alpha_frame_allocator.rs                                                        #
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
//*************************************** DEPENDENCIES *********************************************
//##################################################################################################


use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};


//##################################################################################################
//************************************* STRUCT DECLARATIONS ****************************************
//##################################################################################################


//==================================================================================================
struct AlphaFrameAllocator {
//--------------------------------------------------------------------------------------------------
//  
//==================================================================================================

    current_frame: Frame,
    current_memory_section: Option<&'static MemoryArea>,
    section_iterator: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_info_start: Frame,
    multiboot_info_end: Frame,
}


//##################################################################################################
//************************************ STRUCT IMPLEMENTATIONS **************************************
//##################################################################################################


//==================================================================================================    
impl AlphaFrameAllocator {
//==================================================================================================

}



//==================================================================================================
impl FrameAllocator for AlphaFrameAllocator {
//==================================================================================================


    //==============================================================================================
    fn allocate_frame(&mut self) -> Option<Frame> {
    //----------------------------------------------------------------------------------------------
    // Attempt to allocate a frame.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: Some(...) -> A frame object, if a frame was available
    //          None      -> No frames available
    //==============================================================================================

        match self.current_memory_section {
            None => None,
            Some(section) => {
                let section_frame = Frame::frame_containing_address(section.base_addr as usize);
                
                // Check frame starts within current memory section
                if (self.current_frame.frame_num < Frame::frame_containing_address(section.base_addr as usize).frame_num) {
                    self.current_frame = Frame::frame_containing_address(section.base_addr as usize);
                }
                
                // Check frame ends within current memory section
                if (self.current_frame.frame_num >= Frame::frame_containing_address((section.base_addr+section.length) as usize).frame_num) {
                    self.current_memory_section = self.section_iterator.next();
                    self.allocate_frame()
                }
                // Ensure section is not a kernel section or multiboot info section
                else if (section_frame.frame_num <= self.kernel_end.frame_num &&
                         section_frame.frame_num >= self.kernel_start.frame_num ||
                         section_frame.frame_num >= self.multiboot_info_start.frame_num &&
                         section_frame.frame_num <= self.multiboot_info_end.frame_num) {

                    self.current_memory_section = self.section_iterator.next();
                    self.allocate_frame()
                }
                // Everything checks out
                else {
                    let result: Option<Frame> = Some(self.current_frame);
                    self.current_frame = Frame { frame_num: self.current_frame.frame_num + 1 };
                    result
                }
            }
        }
    }

    
    //==============================================================================================
    fn deallocate_frame(&mut self, frame:Frame) {
    //----------------------------------------------------------------------------------------------
    // Deallocate a frame.
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame -> the frame to deallocate
    //
    // RETURNS: nothing
    //==============================================================================================


    }
}
