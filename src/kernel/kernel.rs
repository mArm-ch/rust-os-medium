#![allow(dead_code)]

use crate::kernel::vga_buffer::print_vga;

/// Represents the core of the operating system.
pub struct Kernel { }

impl Kernel {

    /// Creates a new instance of the Kernel.
    pub fn new() -> Self {
        Kernel { }
    }

    /// Starts the kernel execution loop.
    /// It initializes the display and enters an infinite loop.
    pub fn run(&mut self) -> ! {
        // Prints "Hello World" in cyan
        print_vga("Hello World", 0x3);

        loop {}
    }
}