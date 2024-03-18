	.extern  kernel_trap_entry

	.section .text._trap_entry
	.global  _trap_entry
	.align   2
_trap_entry:
	call     kernel_trap_entry
	j        _trap_entry