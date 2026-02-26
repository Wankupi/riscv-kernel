set disassemble-next-line on
set arch riscv:rv64
add-symbol-file build/os.elf -readnow -o 0x0000000080000000
add-symbol-file build/os.elf -readnow -o 0xffffffff80000000
b *0x80200000
b _trap_entry
b *0x800004f8
b *0x0
