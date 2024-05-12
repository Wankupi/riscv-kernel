
	.section .rodata.elf1
	.align   2
	.global  elf1_start
elf1_start:
	.incbin  "a.out"
	.global  elf1_end
elf1_end: