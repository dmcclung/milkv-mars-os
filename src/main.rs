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

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let hartid: usize;
    unsafe {
        core::arch::asm!("csrr {0}, mhartid", out(reg) hartid);
    }

    if hartid == 0 {
        print("Hello from my rust kernel on JH7110\n");
    }

    loop {}
}
