OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    /* Set the link address to QEMU virt DRAM base */
    . = 0x80000000;

    .text : {
        *(.text.init)       /* Optional: boot code */
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

    /* Discard debug sections if needed */
    /DISCARD/ : {
        *(.debug*)
        *(.comment*)
    }
}