
	.section   .text._kvm_start
	.global    _kvm_start
	.align     2
# a0: address of the mapping table
# a1: v2p offset
_kvm_start:
	add        ra, ra, a1
	la         t0, 1f
	add        t0, t0, a1
	csrw       stvec, t0
	sfence.vma
	csrw       satp, a0
1:
	csrwi      stvec, 0
	ret
