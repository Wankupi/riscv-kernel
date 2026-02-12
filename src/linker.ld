OUTPUT_ARCH(riscv)
ENTRY(_start)
PHYS_RAM_BASE = 0x40000000;
TEXT_OFFSET = 0x00200000;

SECTIONS
{
    . = PHYS_RAM_BASE + TEXT_OFFSET;
    skernel = .;

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
    edata = .;

    ekernel = .;

    . = ALIGN(4K);
    sbss = .;
    .bss (NOLOAD) : {
        *(.bss.stack)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;

    . = ALIGN(4K);

    /DISCARD/ : {
        *(.eh_frame)
    }
}