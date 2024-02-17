OUTPUT_ARCH(riscv)
ENTRY(_start)

KERNEL_BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = KERNEL_BASE_ADDRESS;

    .text : {
        *(.text.entry)
        *(.text .text.*) 
    }
    . = ALIGN(4K);

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);

    .bss : {
        *(.bss .bss.*)
    }
    . = ALIGN(4K);

    /DISCARD/ : {
        *(.comment)
    }
}