
	.global _wait_for_interrupt
	.align  2
_wait_for_interrupt:
# a0: sie
	la      t0, .Lreturn
	csrrw   t0, stvec, t0
	csrrw   a0, sie, a0
	csrsi   sstatus, 2
# sie do not need restore
.Lloop:
	# wfi
	j       .Lloop
	.align  2
.Lreturn:
	csrrw   a0, sie, a0
	csrrw   t0, stvec, t0
	ret
