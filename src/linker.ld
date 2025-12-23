OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    /* Where my kernel starts */
    . = 0x80000000;

    .text : {
        *(.text.init)       /* _start needs to be first */
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
    }

    /* Discard debug sections */
    /DISCARD/ : {
        *(.debug*)
        *(.comment*)
    }
}