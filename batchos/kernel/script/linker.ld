OUTPUT_ARCH(riscv)
ENTRY(_start)

KERNEL_BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = KERNEL_BASE_ADDRESS;

    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*) 
    }
    . = ALIGN(4K);
    etext = .;

    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    erodata = .;

    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    edata = .;

    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    ebss = .;

    ekernel = .;

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.rela*)
        *(.symtab)
    }
}