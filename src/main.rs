#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::kernel::kernel::Kernel;

mod kernel;

/// Entry point of the OS, called by the bootloader.
/// Initializes the kernel and starts the main loop.
#[no_mangle]
pub extern "C" fn _start() -> ! {
	let mut kernel = Kernel::new();
	kernel.run();
}

/// Panic handler: called when a panic occurs.
/// Since this is a bare-metal environment, it enters an infinite loop.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}