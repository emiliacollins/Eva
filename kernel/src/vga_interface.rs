//##################################################################################################
//#                                                                                                #
//# Kernel: vga_inteface                                                                           #
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
//****************************************** DEPENDENCIES ******************************************
//##################################################################################################


use core::ptr::Unique;
use volatile::Volatile;
use spin::Mutex;
use core::fmt;


//##################################################################################################
//************************************** STATIC & CONST DATA ***************************************
//##################################################################################################


const VGA_BUFFER_START : u32    = 0xB8000;
const VGA_NUM_ROWS     : usize  = 25;
const VGA_NUM_COLS     : usize  = 80;


//==================================================================================================


pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    col_position: 0,
    row_position: 0,
    color_fmt: ColorCode::new(VGAColor::Yellow, VGAColor::Black),
    buffer: unsafe { Unique::new(VGA_BUFFER_START as *mut _) },
});


//##################################################################################################
//************************************ STRUCT & ENUM DECLARATIONS **********************************
//##################################################################################################


#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
//==================================================================================================
pub enum VGAColor {
//--------------------------------------------------------------------------------------------------
// VGA/BIOS color codes.
//==================================================================================================
    
    Black      = 0x0,
    Blue       = 0x1,
    Green      = 0x2,
    Cyan       = 0x3,
    Red        = 0x4,
    Magenta    = 0x5,
    Brown      = 0x6,
    LightGray  = 0x7,
    DarkGray   = 0x8,
    LightBlue  = 0x9,
    LightGreen = 0xA,
    LightCyan  = 0xB,
    LightRed   = 0xC,
    Pink       = 0xD,
    Yellow     = 0xE,
    White      = 0xF,
}



#[derive(Debug, Clone, Copy)]
//==================================================================================================
struct ColorCode(u8);
//--------------------------------------------------------------------------------------------------
// Wrapper for 8-bit VGA/BIOS color code. High 4 bits are bg color, low bits are fg color.
//==================================================================================================



#[repr(C)]
#[derive(Debug, Clone, Copy)]
//==================================================================================================
struct VGAChar {
//--------------------------------------------------------------------------------------------------
// Object containing information about a character, its color, and background color.
//==================================================================================================

    character: u8,                      // Character represented
    color: ColorCode,                   // FG/BG color format
}


//==================================================================================================
struct VGABuffer {
//--------------------------------------------------------------------------------------------------
// 
//==================================================================================================
    
    chars: [[Volatile<VGAChar>; VGA_NUM_COLS]; VGA_NUM_ROWS],
}


//==================================================================================================
pub struct Writer {
//--------------------------------------------------------------------------------------------------
//
//==================================================================================================
    
    col_position: usize,
    row_position: usize,
    color_fmt: ColorCode,
    buffer: Unique<VGABuffer>
}


//##################################################################################################
//************************************* STRUCT IMPLEMENTATIONS *************************************
//##################################################################################################


//==================================================================================================
impl VGAChar {
//==================================================================================================

    
    //==============================================================================================
    fn new (c: u8, color: ColorCode) -> VGAChar {
    //----------------------------------------------------------------------------------------------
    // Psuedo-constructor for VGAchar object.
    //----------------------------------------------------------------------------------------------
    // TAKES:   c     -> character VGAChar represents
    //          color -> color formatting for VGAChar
    //
    // RETURNS: VGAChar constructed with given params    
    //==============================================================================================

        VGAChar {
            character: c,
            color: color,
        }
    }
}


//==================================================================================================
impl ColorCode {
//==================================================================================================
    

    //==============================================================================================
    const fn new(fg_color: VGAColor, bg_color: VGAColor) -> ColorCode {
    //----------------------------------------------------------------------------------------------
    // Psuedo-constructor for ColorCode object.
    //----------------------------------------------------------------------------------------------
    // TAKES:   fg_color -> color of character
    //          bg_color -> color of background
    //
    // RETURNS: ColorCode constructed with given params    
    //==============================================================================================
    
        ColorCode(((bg_color as u8) << 4) | (fg_color as u8))
    } 
}


//==================================================================================================
impl Writer {
//==================================================================================================


    //==============================================================================================
    pub fn write_byte(&mut self, byte: u8) {
    //----------------------------------------------------------------------------------------------
    // Writes a single ascii character to the VGA buffer using internal color format.
    //----------------------------------------------------------------------------------------------
    // TAKES:   byte -> ascii character to write
    //
    // RETURNS: nothing    
    //==============================================================================================

        match byte {
            b'\n' => self.new_line(),
            std_char => {
                if (self.col_position >= VGA_NUM_COLS) {
                    self.new_line();
                }

                let col = self.col_position;
                let row = self.row_position;
                let color = self.color_fmt;
                
                self.get_buffer().chars[row][col].write(VGAChar {
                    character: std_char, color: color,
                });
                
                self.col_position += 1;
            }
        }
    }


    //==============================================================================================
    pub fn clear_screen(&mut self) {
    //----------------------------------------------------------------------------------------------
    // Clear the screen of any characters using current background color
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: nothing    
    //==============================================================================================

        let color = self.color_fmt;
        let space = 0x20;

        for i in 0..VGA_NUM_ROWS {
            for j in 0..VGA_NUM_COLS {
                self.get_buffer().chars[i][j].write(VGAChar {
                    character: space, color: color,
                });
            }
        }
        self.row_position = 0;
        self.col_position = 0;
    }


    //==============================================================================================
    fn new_line(&mut self) {
    //----------------------------------------------------------------------------------------------
    // Print a newline to the screen, scrolling up existing output.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: nothing    
    //==============================================================================================

        if self.row_position == ( VGA_NUM_ROWS - 1) {
            let color = self.color_fmt;
            let buffer = self.get_buffer();

            for r in 1..VGA_NUM_ROWS {
                for c in 0..VGA_NUM_COLS {
                    let character = buffer.chars[r][c].read();
                    buffer.chars[r-1][c].write(character);
                }
            }
            
            for c in 0..VGA_NUM_COLS {
                buffer.chars[VGA_NUM_ROWS-1][c].write(VGAChar::new(b' ', color));
            }
        }

        else {
            self.row_position = self.row_position + 1;
        }
        
        self.col_position = 0;   
    }

    
    //==============================================================================================
    fn get_buffer(&mut self) -> &mut VGABuffer {
    //----------------------------------------------------------------------------------------------
    // Get mutable reference to the VGA buffer.
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: Mutable reference to VGA buffer    
    //==============================================================================================
    
        unsafe {self.buffer.get_mut()}
    }
}


//==================================================================================================
impl fmt::Write for Writer {
//==================================================================================================

    //==============================================================================================
    fn write_str(&mut self, string: &str) -> fmt::Result {
    //----------------------------------------------------------------------------------------------
    // 
    //----------------------------------------------------------------------------------------------
    // TAKES:   nothing
    //
    // RETURNS: Mutable reference to VGA buffer    
    //==============================================================================================
        
        for c in string.bytes() {
            self.write_byte(c);
        }
        Ok(())
    }
}


//##################################################################################################
//******************************************** MACROS **********************************************
//##################################################################################################


//==================================================================================================
macro_rules! print {
//----------------------------------------------------------------------------------------------
// Prints a formatted string.
//----------------------------------------------------------------------------------------------
// TAKES:   fmt  -> a string with formatting tokens
//          args -> a series of values; must match number of formatting tokens in fmt    
//==============================================================================================
    
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;

            $crate::vga_interface::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
        }
    }
}


//==================================================================================================
macro_rules! println {
//--------------------------------------------------------------------------------------------------
// Prints a formatted string with a trailing newline.    
//--------------------------------------------------------------------------------------------------
// TAKES:   fmt  -> a string with formatting tokens
//          args -> a series of values; must match number of formatting tokens in fmt    
//==================================================================================================
    
    () => {
        print!("\n");
    };
    ($msg:expr) => {
        print!(concat!($msg, '\n'));
    };
    ($msg:expr, $($arg:tt)*) => {
        print!(concat!($msg, '\n'), $($arg)*);
    };
}


//##################################################################################################
//******************************************** FUNCTIONS *******************************************
//##################################################################################################


pub fn clear_screen() {
    WRITER.lock().clear_screen();
}
