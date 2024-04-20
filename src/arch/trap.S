#define TRAP_FRAME_ADDR (0x0 - 0x1000 - 0x1000)
#define OFFSET_REGS 0x28
	.extern    kernel_trap_entry

	.section   .text._trap_entry_early
	.global    _trap_entry_early
	.align     2
_trap_entry_early:
	call       kernel_trap_entry
	j          _trap_entry

	.section   .text.trampoline
	.global    _trap_entry
	.align     2
_trap_entry:
	csrw       sscratch, a0
	li         a0, TRAP_FRAME_ADDR
	add        a0, a0, OFFSET_REGS
	sd         x1, 0(a0)
	sd         x2, 8(a0)
	sd         x3, 16(a0)
	sd         x4, 24(a0)
	sd         x5, 32(a0)
	sd         x6, 40(a0)
	sd         x7, 48(a0)
	sd         x8, 56(a0)
	sd         x9, 64(a0)
	csrr       t0, sscratch
	sd         t0, 72(a0)
	sd         x11, 80(a0)
	sd         x12, 88(a0)
	sd         x13, 96(a0)
	sd         x14, 104(a0)
	sd         x15, 112(a0)
	sd         x16, 120(a0)
	sd         x17, 128(a0)
	sd         x18, 136(a0)
	sd         x19, 144(a0)
	sd         x20, 152(a0)
	sd         x21, 160(a0)
	sd         x22, 168(a0)
	sd         x23, 176(a0)
	sd         x24, 184(a0)
	sd         x25, 192(a0)
	sd         x26, 200(a0)
	sd         x27, 208(a0)
	sd         x28, 216(a0)
	sd         x29, 224(a0)
	sd         x30, 232(a0)
	sd         x31, 240(a0)
	csrr       t0, sepc
	sd         t0, 248(a0)
	addi       a0, a0, -OFFSET_REGS
	ld         a1, 0(a0)               # kernel satp
	ld         sp, 8(a0)               # kernel sp
	ld         ra, 16(a0)              # kernel trap function
	ld         tp, 24(a0)              # hart id
	sfence.vma zero, zero
	csrw       satp, a1
	sfence.vma zero, zero
	jr         ra                      # jump to kernel trap function

	.global    _user_ret
	.align     2
# _user_ret(a0: user satp)
_user_ret:
	li         a0, TRAP_FRAME_ADDR
# satp
	ld         a1, 0x20(a0)
	add        a0, a0, OFFSET_REGS
	sfence.vma zero, zero
	csrw       satp, a1
	sfence.vma zero, zero
# pc
	ld         t0, 248(a0)
	csrw       sepc, t0
# regs
	ld         x1, 0(a0)
	ld         x2, 8(a0)
	ld         x3, 16(a0)
	ld         x4, 24(a0)
	ld         x5, 32(a0)
	ld         x6, 40(a0)
	ld         x7, 48(a0)
	ld         x8, 56(a0)
	ld         x9, 64(a0)
	ld         x11, 80(a0)
	ld         x12, 88(a0)
	ld         x13, 96(a0)
	ld         x14, 104(a0)
	ld         x15, 112(a0)
	ld         x16, 120(a0)
	ld         x17, 128(a0)
	ld         x18, 136(a0)
	ld         x19, 144(a0)
	ld         x20, 152(a0)
	ld         x21, 160(a0)
	ld         x22, 168(a0)
	ld         x23, 176(a0)
	ld         x24, 184(a0)
	ld         x25, 192(a0)
	ld         x26, 200(a0)
	ld         x27, 208(a0)
	ld         x28, 216(a0)
	ld         x29, 224(a0)
	ld         x30, 232(a0)
	ld         x31, 240(a0)
	ld         a0, 72(a0)
	sret
