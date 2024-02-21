    .section .text.loader
    .globl _start
    .globl boot_stack_top
    .globl boot_stack_lower_bound
_start:
    la sp, boot_stack_top
    call start_kernel

    .section .bss.stack
boot_stack_lower_bound:
    .space 4096 * 16
boot_stack_top: