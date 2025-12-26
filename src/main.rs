// main.rs

#![no_std]
#![no_main]

const UART: usize = 0x10000000;

fn putc(c: u8) {
    unsafe {
        core::ptr::write_volatile(UART as *mut u8, c);
        // LSR never worked for me on the Milkv mars
        // so I used a delay
        for _ in 0..50000 {
            core::arch::asm!("nop");
        }
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
        "la sp, _stack_top",
        
        "call kmain",
        "1:",
        "wfi",
        "j 1b",
    )
}

#[unsafe(no_mangle)]
unsafe fn kmain() -> ! {
    print("Hello from Rust kernel on JH7110!\n");
    print("Milk-V Mars is running!\n");
    
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
