
// VGA buffer memory address
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

/// Prints a string in a color
pub fn print_vga(s: &str, color: u8) {
    let mut offset = 0;

    // Process each byte of the string
    for byte in s.bytes() {
        unsafe {
            // Each chars takes 2 bits, 1 for the charcode, 
            // second for the color
            *VGA_BUFFER.add(offset) = byte;
            *VGA_BUFFER.add(offset + 1) = color;
        }

        // Move to next char
        offset += 2;
    }
}