OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
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

    . = ALIGN(4K);
    sbss = .;
    .bss : {
        *(.bss.stack)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    ebss = .;

    ekernel = .;
    . = ALIGN(4K);

    # uart_base_addr = 0x10000000;
    # system_reset_addr = 0x100000;

    /DISCARD/ : {
        *(.eh_frame)
    }
}