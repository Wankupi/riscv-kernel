set disassemble-next-line on
set arch riscv:rv64
add-symbol-file build/os.elf -readnow -o 0xffffffff00000000
