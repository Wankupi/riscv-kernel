	.extern  solve_trap

	.section .text._trap_entry
	.global  _trap_entry
	.align   2
_trap_entry:
	call     solve_trap
	j        _trap_entry