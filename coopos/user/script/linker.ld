OUTPUT_ARCH(riscv)
ENTRY(_start)

USER_BASE_ADDRESS = 0x80400000;

SECTIONS
{
    . = USER_BASE_ADDRESS;

    .text : {
        *(.text.entry)
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
        start_bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        end_bss = .;
    }

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.rela*)
        *(.symtab)
    }
}