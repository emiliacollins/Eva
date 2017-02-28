//##################################################################################################
//#                                                                                                #
//# Kernel/memory/paging: table.rs                                                                 #
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


use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use core::ops::{Index,IndexMut};
use core::marker::PhantomData;


//##################################################################################################
//****************************************** CONSTANTS *********************************************
//##################################################################################################


const PAGE_MAP:   *mut Table<PageMap> = 0xFFFF_FFFF_FFFF_F000 as *mut _;


//##################################################################################################
//********************************************* TRAITS *********************************************
//##################################################################################################


//==================================================================================================
pub trait TableLevel {}
//--------------------------------------------------------------------------------------------------
// Marker indicating that object is a valid table in the multi-level paging system.
//==================================================================================================


//==================================================================================================
pub trait MetaLevel: TableLevel {
//--------------------------------------------------------------------------------------------------
// Marker indicating that object is a table that points to other tables, rather than to pages. 
//==================================================================================================
    
    type NextLevel: TableLevel;         // The type of table that entries point to
}


//##################################################################################################
//**************************************** ENUM DECLARATIONS ***************************************
//##################################################################################################


//==================================================================================================
pub enum PageMap {}
pub enum PointerTable {}
pub enum PageDirectory {}
pub enum PageTable {}
//--------------------------------------------------------------------------------------------------
// Abstract types used to make table manipulation safer by differentiating between tables.
//==================================================================================================


//##################################################################################################
//************************************** STRUCT DECLARATIONS ***************************************
//##################################################################################################


//==================================================================================================
struct Table<Level: TableLevel> {
//--------------------------------------------------------------------------------------------------
// Allows read and write access to table in multi-level paging system.
//==================================================================================================
    
    entries: [PageEntry; ENTRY_COUNT],  // Create ENTRY_COUNT entries in the table
    level: PhantomData<Level>,          // Tell rust to chill about unused type param
}


//##################################################################################################
//*************************************** ENUM IMPLEMENTATIONS *************************************
//##################################################################################################


//==================================================================================================
impl TableLevel for PageMap {}
impl TableLevel for PointerTable {}
impl TableLevel for PageDirectory {}
impl TableLevel for PageTable {}
//--------------------------------------------------------------------------------------------------
// Ensure all tables may be used in functions usable on any type of table.
//==================================================================================================


//==================================================================================================
impl MetaLevel for PageMap       { type NextLevel = PointerTable;  }
impl MetaLevel for PointerTable  { type NextLevel = PageDirectory; }
impl MetaLevel for PageDirectory { type NextLevel = PageTable;     }
//--------------------------------------------------------------------------------------------------
// Ensure only non PageTable tables are used in certain table manipulation functions.
//==================================================================================================


//##################################################################################################
//************************************* STRUCT IMPLEMENTATIONS *************************************
//##################################################################################################


//==================================================================================================
impl<Level: TableLevel> Table<Level>  {
//==================================================================================================


    //==============================================================================================
    fn clear(&mut self) {
    //----------------------------------------------------------------------------------------------
    // Empty the page table of valid entries by unsetting all PRESENT bits.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: nothing
    //==============================================================================================
        
        for entry in self.entries.iter_mut() {
            entry.mark_unused();
        }
    }   
}


//=================================================================================================
impl <Level: MetaLevel> Table<Level> {
//=================================================================================================


    //=============================================================================================
    fn next_table_addr(&self, index: usize) -> Option<usize> {
    //----------------------------------------------------------------------------------------------
    // Calculate the starting address of the table pointed to in the indexth entry. 
    //----------------------------------------------------------------------------------------------
    // TAKES:   index -> index of the entry to examine
    //
    // RETURNS: Some(...) -> address of the table pointed to in the indexth entry
    //          None      -> entry was inactive, no valid table address
    //==============================================================================================

        if (self[index].flags().contains(PRESENT)) {
            Some((self as *const _ as usize) << 9 | index << 12)
        }
        else {
            None
        }
    }

    
    //==============================================================================================
    pub fn next_table(&self, index: usize) -> Option<&Table<Level::NextLevel>> {
    //----------------------------------------------------------------------------------------------
    // Obtain an immutable reference to the table pointed to by the indexth entry.
    //----------------------------------------------------------------------------------------------
    // TAKES:   index -> index of the entry to examine
    //
    // RETURNS: Some(...) -> immutable reference to the table pointed to in the indexth entry.
    //          None      -> index pointed to an inactive entry    
    //==============================================================================================

        self.next_table_addr(index).map(|addr| unsafe {&*(addr as *const _)})
    }


    //==============================================================================================
    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<Level::NextLevel>> {
    //----------------------------------------------------------------------------------------------
    // Obtain an mutable reference to the table pointed to by the indexth entry.
    //----------------------------------------------------------------------------------------------
    // TAKES:   index -> index of the entry to examine
    //
    // RETURNS: Some(...) -> mutable reference to the table pointed to in the indexth entry.
    //          None      -> index pointed to an inactive entry    
    //==============================================================================================

        self.next_table_addr(index).map(|addr| unsafe {&mut *(addr as *mut _)})
    }    
}


//==================================================================================================
impl<Level: TableLevel> Index<usize> for Table<Level> {
//==================================================================================================

    
    //==============================================================================================
    type Output = PageEntry;
    //----------------------------------------------------------------------------------------------
    // Type to be returned when indexing.
    //==============================================================================================
    

    //==============================================================================================
    fn index(&self, index: usize) -> &Self::Output {
    //----------------------------------------------------------------------------------------------
    // Obtain reference to index-th entry in the table.
    //----------------------------------------------------------------------------------------------
    // TAKES:   index -> index of desired entry
    //
    // RETURNS: reference to the desired entry
    //==============================================================================================
        
        &self.entries[index]
    }
}


//==================================================================================================
impl<Level: TableLevel> IndexMut<usize> for Table<Level> {
//==================================================================================================


    //==============================================================================================
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    //----------------------------------------------------------------------------------------------
    // Obtain mutable reference to index-th entry in the table.
    //----------------------------------------------------------------------------------------------
    // TAKES:   index -> index of desired entry
    //
    // RETURNS: mutable reference to the desired entry
    //==============================================================================================

        &mut self.entries[index]
    }
}
