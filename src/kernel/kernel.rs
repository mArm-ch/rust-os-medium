#![allow(dead_code)]

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
        // Here goes the kernel code
        loop {}
    }
}