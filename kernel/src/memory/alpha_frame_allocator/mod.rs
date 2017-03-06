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


use memory::{Frame, FrameAllocator, PAGE_SIZE};
use multiboot2::{MemoryArea, MemoryAreaIter};


//##################################################################################################
//************************************* STRUCT DECLARATIONS ****************************************
//##################################################################################################


//==================================================================================================
pub struct AlphaFrameAllocator {
//--------------------------------------------------------------------------------------------------
//  
//==================================================================================================

    curr_frame_addr: usize,
    curr_section: Option<&'static MemoryArea>,
    section_iterator: MemoryAreaIter,
    kernel_start: usize,
    kernel_length: usize,
    multiboot_start: usize,
    multiboot_length: usize,
}


//##################################################################################################
//************************************ STRUCT IMPLEMENTATIONS **************************************
//##################################################################################################


//==================================================================================================    
impl AlphaFrameAllocator {
//==================================================================================================


    //==============================================================================================
    pub fn new(kernel_start: usize, kernel_length: usize, multiboot_start: usize, multiboot_length: usize,
               mut memory_areas: MemoryAreaIter) -> AlphaFrameAllocator {
    //----------------------------------------------------------------------------------------------
    // Pseudo-constructor for AlphaFrameAllocator
    //----------------------------------------------------------------------------------------------
    // TAKES:   kernel_start     -> starting address of the kernel
    //          kernel_length    -> length of kernel memory
    //          multiboot_start  -> starting address of the multiboot information structure
    //          multiboot_length -> length of multiboot information structure
    //          memory_areas     -> iterator over sections of memory
    //    
    // RETURNS: a frame allocator that cannot free up frames, only allocate frames
    //==============================================================================================

        AlphaFrameAllocator {
            curr_frame_addr: 0,
            curr_section: memory_areas.next(),
            section_iterator: memory_areas,
            kernel_start: kernel_start,
            kernel_length: kernel_length,
            multiboot_start: multiboot_start,
            multiboot_length: multiboot_length,
                
        }
    }

    
    //==============================================================================================
    fn section_needed_correction(&mut self) -> bool {
    //----------------------------------------------------------------------------------------------
    // Fixes the current section if necessary, and reports back if a correction was made.
    //----------------------------------------------------------------------------------------------    
    // TAKES:   nothing
    // 
    // RETURNS: true  -> a change was made to self.curr_section
    //          false -> no changes were made
    //==============================================================================================
        
        match self.curr_section {
            None => false,
            Some(section) => {
                // Check if frame has passed last full frame in section
                if (self.curr_frame_addr >= (section.base_addr + section.length) as usize / PAGE_SIZE * PAGE_SIZE + PAGE_SIZE) {

                    // Increment the section
                    self.curr_section = self.section_iterator.next();
                    true;
                }

                false
            }
        }
    }


    //==============================================================================================
    fn frame_needed_correction(&mut self) -> bool {
    //----------------------------------------------------------------------------------------------
    // Fixes the current frame if necessary, and reports back if a correction was made.
    //----------------------------------------------------------------------------------------------    
    // TAKES:   nothing
    // 
    // RETURNS: true  -> a change was made to self.curr_frame_addr
    //          false -> no changes were made
    //==============================================================================================

        match self.curr_section {
            None => false,
            Some(section) => {
                // Check if frame needs to be fast-forwarded to new section
                if (self.curr_frame_addr < section.base_addr as usize) {

                    // Fast-forward frame to first full frame in section
                    self.curr_frame_addr = (section.base_addr as usize + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;
                    return true
                }
                // Check that frame does not contain kernel or multiboot memory
                else if (self.curr_frame_addr >= self.kernel_start && self.curr_frame_addr < self.kernel_start + self.kernel_length ||
                         self.curr_frame_addr >= self.multiboot_start && self.curr_frame_addr < self.multiboot_start + self.multiboot_length) {
                
                    // Increment the frame
                    self.curr_frame_addr = self.curr_frame_addr + PAGE_SIZE;
                    return true
                }

                // Frame did not need correction ls
                false
            },
        }
    }
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

        // Fix frame and section as necessary
        while (AlphaFrameAllocator::section_needed_correction(self) || AlphaFrameAllocator::frame_needed_correction(self)) {};

        // Either section iterator ran out, or frame points at valid frame
        match self.curr_section {
            None => None,
            _ => {
                let result = Frame { frame_num: self.curr_frame_addr / PAGE_SIZE };
                self.curr_frame_addr = self.curr_frame_addr + PAGE_SIZE;
                Some(result)
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

        //unimplemented!();
    }
}
