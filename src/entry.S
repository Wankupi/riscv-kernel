# Linker Constants
    .extern    skernel
    .extern    ekernel
    .extern    TEXT_OFFSET
# External Symbols
    .extern    kmain_early
    .extern    kmain
    .extern    dtb_addr
    .extern    kvm_config
    .section   .text.entry
    .globl     _start
_start:
# Image Header
    j          .L_start
    j          _start
    .balign    8
    .quad      TEXT_OFFSET /* text_offset */
    .quad      ekernel - skernel /* image_size */
    .quad      0 /* flags */
    .word      2 /* version */
    .word      0 /* res1 */
    .quad      0 /* res2 */
    .quad      0x5643534952 /* magic "RISCV" */
    .word      0x05435352 /* magic2 "RSC\x05" */
    .word      0 /* res3 */
.L_start:
    sd         a1, dtb_addr, t0
    la         sp, boot_stack_top
    call       kmain_early

    call       _enable_mmu

    la         sp, boot_stack_top
    tail       kmain

    .section   .text.enable_mmu
_enable_mmu:
    la         t0, kvm_config
    ld         a0, 0(t0)                          # satp
    ld         a1, 8(t0)                          # offset of virt and phys
    add        ra, ra, a1
    la         a2, 1f
    add        a2, a2, a1
    csrw       stvec, a2
    sfence.vma
    csrw       satp, a0
    .align     2
1:
    sfence.vma
    csrw       stvec, 0
    ret


    .section   .bss.stack.boot_stack_lower_bound
    .globl     boot_stack_lower_bound
boot_stack_lower_bound:
    .space     4096 * 64
    .section   .bss.stack.boot_stack_top
    .globl     boot_stack_top
boot_stack_top:
