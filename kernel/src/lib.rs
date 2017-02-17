//##################################################################################################
//#                                                                                                #
//# Kernel: lib.rs                                                                                 #
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
//************************************** LIBRARY ATTRIBUTES ****************************************
//##################################################################################################


#![feature(lang_items)] // allows us access to feature-gated modifications to core desugared functions
#![feature(unique)]
#![feature(const_fn)]
#![no_std]              // disallow linking to standard libraries, we need to be static


//##################################################################################################
//****************************************** DEPENDENCIES ******************************************
//##################################################################################################


extern crate rlibc;                     // minimal libc-style ops 
extern crate volatile;                  // mark operations as volatile to prevent optimizations
extern crate spin;                      // minimal "busy-loop" mutex support
extern crate multiboot2;                // module to parse multiboot v2 info from memory


//==================================================================================================


#[macro_use]
pub mod vga_interface;                  // interface for more easily interacting with vga buffer
pub mod memory;


//##################################################################################################
//***************************** BOILER-PLATE NO_STDLIB REQUIREMENTS ********************************
//##################################################################################################


#[allow(non_snake_case)]
#[no_mangle]
//==================================================================================================
pub extern "C" fn _Unwind_Resume() -> ! {
//--------------------------------------------------------------------------------------------------
// Overrides panic unwinding, as it relies on os libraries which will be unavailable.
//--------------------------------------------------------------------------------------------------
// TAKES:   nothing
//
// RETURNS: never
//==================================================================================================

    loop {}
}


#[lang = "eh_personality"]
//==================================================================================================
extern fn eh_personality() {
//--------------------------------------------------------------------------------------------------
// Used in compiler failure mechanism. Never called if panic! does not occur.
//--------------------------------------------------------------------------------------------------
// TAKES:   nothing
//
// RETURNS: nothing
//==================================================================================================


}


#[lang = "panic_fmt"]
#[no_mangle]
//==================================================================================================
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32)  -> ! {
//--------------------------------------------------------------------------------------------------
// Prints a message handed to the panic! macro, as well as info about where the panic! occured.
//--------------------------------------------------------------------------------------------------
// TAKES:   fmt  -> panic message as a series of arguments
//          file -> file in which panic occured
//          line -> line on which panic occured
//
// RETURNS: never
//==================================================================================================

    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("   {}", fmt);
    loop {};
}


//##################################################################################################
//*********************************************** MAIN *********************************************
//##################################################################################################


#[no_mangle]                            
//==================================================================================================
pub extern fn rust_main(multiboot_info_start: usize) {
//==================================================================================================

    vga_interface::WRITER.lock().clear_screen();
    
    let boot_info = unsafe {multiboot2::load(multiboot_info_start)};
    let memory_map_tag = boot_info.memory_map_tag().expect("No memory map tag found!");

    println!("Memory Areas: ");

    for area in memory_map_tag.memory_areas() {
        println!("start: {:x} -> end: {:x}", area.base_addr, area.base_addr + area.length);
    }

    println!("Kernel Sections: ");
    
    let elf_sections_tag = boot_info.elf_sections_tag().expect("No kernel-elf tag found!");
    for section in elf_sections_tag.sections() {
        println!("addr: 0x{:x}, size 0x{:}, flags:0x{:x}",section.addr,section.size,section.flags);
    }
    

    loop {}
}

