	.section .text._switch
	.global  _switch
	.align   2
_switch:
# a0 = current context
# a1 = next context
# save current context
	sw       ra, 0x0(a0)
	sw       sp, 0x8(a0)
	sw       s0, 0x18(a0)
	sw       s1, 0x20(a0)
	sw       s2, 0x28(a0)
	sw       s3, 0x30(a0)
	sw       s4, 0x38(a0)
	sw       s5, 0x40(a0)
	sw       s6, 0x48(a0)
	sw       s7, 0x50(a0)
	sw       s8, 0x58(a0)
	sw       s9, 0x60(a0)
	sw       s10, 0x68(a0)
	sw       s11, 0x70(a0)
# restore next context
	lw       ra, 0x0(a1)
	lw       sp, 0x8(a1)
	lw       s0, 0x18(a1)
	lw       s1, 0x20(a1)
	lw       s2, 0x28(a1)
	lw       s3, 0x30(a1)
	lw       s4, 0x38(a1)
	lw       s5, 0x40(a1)
	lw       s6, 0x48(a1)
	lw       s7, 0x50(a1)
	lw       s8, 0x58(a1)
	lw       s9, 0x60(a1)
	lw       s10, 0x68(a1)
	lw       s11, 0x70(a1)
# return to next context
	ret
