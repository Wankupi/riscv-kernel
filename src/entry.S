    .extern    kmain_early
    .extern    kmain
    .extern    dtb_addr
    .extern    kvm_config
    .section   .text.entry
    .globl     _start
_start:
    sd         a1, dtb_addr, t0
    la         sp, boot_stack_top
    call       kmain_early

    call       _enable_mmu

    la         sp, boot_stack_top
    tail       kmain

    .section   .text.enable_mmu
_enable_mmu:
    la         t0, kvm_config
    ld         a0, 0(t0)              # offset of virt and phys
    ld         a1, 8(t0)              # page table address
    add        ra, ra, a0
    la         a2, 1f
    add        a2, a2, a0
    csrw       stvec, a2
    sfence.vma
    csrw       satp, a1
1:
    sfence.vma
    csrw       stvec, 0
    ret

    .section   .bss.stack
    .globl     boot_stack_lower_bound
boot_stack_lower_bound:
    .space     4096 * 64
    .globl     boot_stack_top
boot_stack_top:
