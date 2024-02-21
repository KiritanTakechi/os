OUTPUT_ARCH(riscv)
ENTRY(_start)

KERNEL_LINK_ADDR = 0x80200000;

SECTIONS
{
    . = KERNEL_LINK_ADDR;

    .text : {
        *(.text.loader)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : {
        *(.bss.stack)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    /DISCARD/ : {
        *(.eh_frame)
    }
}