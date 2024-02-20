.globl _start
.globl boot_stack_lower_bound
.globl boot_stack_top

.section .text.entry
_start:
    la sp, boot_stack_top

    call start_kernel

.section .bss.stack
boot_stack_lower_bound:
    .space 4096 * 32
boot_stack_top: