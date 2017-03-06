//##################################################################################################
//#                                                                                                #
//# Kernel/memory/paging: entry.rs                                                                 #
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
//***************************************** DEPENDENCIES *******************************************
//##################################################################################################


use memory::Frame;


//##################################################################################################
//****************************************** CONSTANTS *********************************************
//##################################################################################################


const PAGE_ALIGNED_52_BIT_MASK: usize = 0x000F_FFFF_FFFF_F000;


//##################################################################################################
//************************************* STRUCT DECLARATIONS ****************************************
//##################################################################################################


//==================================================================================================
pub struct PageEntry(u64);
//--------------------------------------------------------------------------------------------------
// Wrapper for 64-bit value to be used as an entry in a page table.
//==================================================================================================


//==================================================================================================
bitflags! { pub flags EntryFlags: u64 {
//--------------------------------------------------------------------------------------------------
// Flags contained in a page entry.   
//==================================================================================================
    
    const PRESENT      = 1 << 0,        // Target exists
    const WRITABLE     = 1 << 1,        // Target may be written to
    const ACCESSIBLE   = 1 << 2,        // Target may be accessable
    const WRITETHROUGH = 1 << 3,        // Bypass cache when writing
    const NO_CACHE     = 1 << 4,        // Disable caching entirely
    const ACCESSED     = 1 << 5,        // Target has been used
    const DIRTY        = 1 << 6,        // Target has been written to
    const HUGE_PAGE    = 1 << 7,        // Target size
    const GLOBAL       = 1 << 8,        // Keep target in cache on addr space switch
    const NO_EXEC      = 1 << 63,       // Forbid executing code in target
  }
}


//##################################################################################################
//*********************************** STRUCT IMPLEMENTATIONS ***************************************
//##################################################################################################


//==================================================================================================
impl PageEntry {
//==================================================================================================


    //==============================================================================================
    pub fn is_unused(&self) -> bool {
    //----------------------------------------------------------------------------------------------
    // Checks whether PRESENT bit is set in the entry.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: true  -> PRESENT bit set, entry present
    //          false -> Present bit not set, no entry    
    //==============================================================================================

        self.flags().contains(PRESENT)
    }


    //==============================================================================================
    pub fn mark_unused(&mut self)  {
    //----------------------------------------------------------------------------------------------
    // Marks an entry as unused.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: nothing
    //==============================================================================================

        let mut flags = self.flags();
        flags.remove(PRESENT);
        self.set_flags(flags);
    }


    //==============================================================================================
    pub fn flags(&self) -> EntryFlags {
    //----------------------------------------------------------------------------------------------
    // Obtain a copy of this entry's flags.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: EntryFlag object containing copy of flag states in this entry
    //==============================================================================================

        EntryFlags::from_bits_truncate(self.0)
    }


    //==============================================================================================
    pub fn target_frame(&self) -> Option<Frame> {
    //----------------------------------------------------------------------------------------------
    // Obtain frame referenced by entry.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: Some(...) -> Frame object corresponding to address in entry
    //          None      -> Entry is invalid, no corresponding frame    
    //==============================================================================================

        if (self.flags().contains(PRESENT)) {
            Some(Frame::frame_containing_address(
                self.0 as usize & PAGE_ALIGNED_52_BIT_MASK))
        }
        else {
            None
        }
    }


    //==============================================================================================
    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
    //----------------------------------------------------------------------------------------------
    // Set this entry's corresponding frame and flags.
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame -> Frame entry should point to
    //          flags -> flags to be set in entry
    //    
    // RETURNS: nothing
    //==============================================================================================

        self.0 = ((frame.address() as u64) | flags.bits());
    }

    
    //==============================================================================================
    pub fn set_frame(&mut self, frame: Frame) {
    //----------------------------------------------------------------------------------------------
    // Set this entry's corresponding frame.
    //----------------------------------------------------------------------------------------------
    // TAKES:   frame -> Frame entry should point to
    //
    // RETURNS: nothing
    //==============================================================================================


        self.0 = (self.flags().bits() | frame.address() as u64);  
    }

    //==============================================================================================
    pub fn set_flags(&mut self, flags: EntryFlags) {
    //----------------------------------------------------------------------------------------------
    // Set this entry's flags.
    //----------------------------------------------------------------------------------------------
    // TAKES:   flags -> flags to be set in entry
    //
    // RETURNS: nothing
    //==============================================================================================

        self.0 = match self.target_frame() {
            Some(frame) => { (frame.address() as u64) | flags.bits() },
            None        => { flags.bits() }, 
        };
    }
}
