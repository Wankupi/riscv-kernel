OUTPUT_ARCH(riscv)
ENTRY(_start)
PHYS_RAM_BASE = 0x00000000;
TEXT_OFFSET = 0x00200000;

SECTIONS
{
    . = PHYS_RAM_BASE + TEXT_OFFSET;
    skernel = .;
    kernel_load_base = . - TEXT_OFFSET;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4K);
        *(.text.trampoline)
        . = ALIGN(4K);
        *(.text .text.*)
    }
    etext = .;

    . = ALIGN(4K);
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    erodata = .;

    . = ALIGN(4K);
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .rel.dyn : {
		*(.rel.dyn*)
	}
	.rela.dyn : ALIGN(8) {
		__rela_dyn_start = .;
		*(.rela .rela*)
		__rela_dyn_end = .;
	}
    edata = .;
    ekernel_image = .;

    . = ALIGN(4K);
    sbss = .;
    .bss (NOLOAD) : {
        *(.bss.stack)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;

    ekernel = .;
    . = ALIGN(4K);

    /DISCARD/ : {
        *(.eh_frame)
    }
}