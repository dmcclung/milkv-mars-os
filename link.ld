OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80200000;
    
    .text : {
        *(.text.init)       /* _start must be first */
        *(.text*)
    }
    
    .rodata : {
        *(.rodata*)
    }
    
    .data : {
        *(.data*)
    }
    
    .bss : {
        _bss_start = .;
        *(.bss*)
        *(.sbss*)
        _bss_end = .;
    }
    
    /* Stack grows downward from this address */
    . = ALIGN(16);
    . += 0x10000;  /* 64KB stack */
    _stack_top = .;
    
    /DISCARD/ : {
        *(.debug*)
        *(.comment*)
    }
}
