// main.rs

#![no_std]
#![no_main]

const UART0: usize = 0x10000000;
const LSR_OFFSET: usize = 5; // Line status register offset

fn putc(c: u8) {
    unsafe {
        // Wait until the Transmitter Holding Register (THRE) is empty
        // 1 = transmitter is ready
        // 0 = transmitter is busy
        while (core::ptr::read_volatile((UART0 + LSR_OFFSET) as *const u8) & (1 << 5)) == 0 {
            core::arch::asm!("nop");
        }
        core::ptr::write_volatile(UART0 as *mut u8, c);
    }
}

fn print(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            putc(b'\r');
        }
        putc(byte);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = ".text.init")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        // Set up stack pointer
        "la sp, _stack_top",
        
        // Clear BSS (if you have any globals later)
        "la t0, _bss_start",
        "la t1, _bss_end",
        "1:",
        "bgeu t0, t1, 2f",
        "sd zero, 0(t0)",
        "addi t0, t0, 8",
        "j 1b",
        "2:",
        
        // Jump to Rust code
        "call kmain",
        "j .",  // Hang if kmain returns
    )
}

#[unsafe(no_mangle)]
fn kmain() -> ! {
    unsafe {
            // Try all possible UART addresses
            let uarts = [
                0x10000000,  // UART0
                0x10010000,  // UART1
                0x10020000,  // UART2
                0x12000000,  // UART3 (often on GPIO)
                0x12010000,  // UART4
                0x12020000,  // UART5
            ];
            
            for (i, &uart_base) in uarts.iter().enumerate() {
                let uart = uart_base as *mut u8;
                
                // Write "UARTx: Hello\r\n"
                let msg = [b'U', b'A', b'R', b'T', b'0' + i as u8, b':', b' ', 
                           b'H', b'e', b'l', b'l', b'o', b'\r', b'\n'];
                
                for &c in &msg {
                    core::ptr::write_volatile(uart, c);
                    
                    // Delay between characters
                    for _ in 0..1000000 {
                        core::arch::asm!("nop");
                    }
                }
            }
        }
    loop {}
}
