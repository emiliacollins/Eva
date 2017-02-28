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
pub struct Page {
//--------------------------------------------------------------------------------------------------
// Object representing a memory page.    
//==================================================================================================
    
    page_num: usize,                    // Page index
}



