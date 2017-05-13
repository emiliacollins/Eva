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
#![allow(unused_parens)]


//##################################################################################################
//****************************************** DEPENDENCIES ******************************************
//##################################################################################################


extern crate rlibc;                     // minimal libc-style ops 
extern crate volatile;                  // mark operations as volatile to prevent optimizations
extern crate spin;                      // minimal "busy-loop" mutex support
extern crate multiboot2;                // module to parse multiboot v2 info from memory
#[macro_use]
extern crate bitflags;                  // bitflags used in paging
extern crate x86;


//==================================================================================================


#[macro_use]
pub mod vga_interface;                  // interface for more easily interacting with vga buffer
mod memory;


//==================================================================================================


use memory::AlphaFrameAllocator;
use x86::shared::msr::{IA32_EFER,rdmsr,wrmsr};
use x86::shared::control_regs::{cr0,cr0_write,CR0_WRITE_PROTECT};


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
//********************************************* UTILITIES ******************************************
//##################################################################################################


//==================================================================================================
fn enable_write_protection() {
//--------------------------------------------------------------------------------------------------
//    
//--------------------------------------------------------------------------------------------------
//
//
//
//==================================================================================================

    unsafe {
        wrmsr(IA32_EFER, rdmsr(IA32_EFER) | 1 << 11); 
        cr0_write(cr0() | CR0_WRITE_PROTECT);
    }
}

//##################################################################################################
//*********************************************** MAIN *********************************************
//##################################################################################################


#[no_mangle]                            
//==================================================================================================
pub extern fn rust_main(multiboot_info_start: usize) {
//==================================================================================================

    vga_interface::WRITER.lock().clear_screen();

    let boot_info = unsafe { multiboot2::load(multiboot_info_start) };

    let memory_map_tag = boot_info.memory_map_tag().expect("Need memory map tag!");

    let elf_sections_tag = boot_info.elf_sections_tag().expect("Need ELF sections tag");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();

    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
    
    let multiboot_end = multiboot_info_start + (boot_info.total_size as usize);

    let mut frame_allocator = memory::AlphaFrameAllocator::new(kernel_start as usize, kernel_end as usize,
                                                               multiboot_info_start, multiboot_end,
                                                               memory_map_tag.memory_areas());
    
    enable_write_protection();

    memory::paging::remap_kernel(&mut frame_allocator, boot_info);

    println!("It works!");
    use memory::FrameAllocator;
    frame_allocator.allocate_frame();
    
    println!("Still working!");
    loop {}
}

