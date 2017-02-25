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

use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use core::ops::{Index,IndexMut};


const PAGE_MAP:   *mut Table = 0xFFFF_FFFF_FFFF_F000 as *mut Table;
const PTR_TABLE:  *mut Table = 0xFFFF_FFFF_FFE0_0000 as *mut Table;
const PAGE_DIR:   *mut Table = 0xFFFF_FFFF_C000_0000 as *mut Table;
const PAGE_TABLE: *mut Table = 0xFFFF_FF80_0000_0000 as *mut Table; 


struct Table {
    entries: [PageEntry; ENTRY_COUNT],
}



impl Table {
    fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.mark_unused();
        }
    }

    fn next_table_addr(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if (entry_flags.contains(PRESENT)) {
            Some((self as *const _ as usize) << 9 | index << 12)
        }
        else {
            None
        }
    }

    pub fn next_table(&self, index: usize) -> Option<&Table> {
        self.next_table_addr(index).map(|addr| unsafe {&*(addr as *const _)})
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.next_table_addr(index).map(|addr| unsafe {&mut *(addr as *mut _)})
    }

    

    
}


impl Index<usize> for Table {
    type Output = PageEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }

}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}
