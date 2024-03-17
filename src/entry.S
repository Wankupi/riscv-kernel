    .extern kmain
    .extern dtb_addr
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    sd a1, dtb_addr, t0
    call kmain

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 64
    .globl boot_stack_top
boot_stack_top:
