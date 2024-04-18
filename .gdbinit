set disassemble-next-line on
set arch riscv:rv64
add-symbol-file build/os.elf -readnow -o 0x0000000000000000
add-symbol-file build/os.elf -readnow -o 0xffffffff00000000

b kmain
b kernel_trap_entry
