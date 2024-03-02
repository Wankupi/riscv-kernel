testspace = testspace

CARGO = cargo
OBJCOPY = riscv64-elf-objcopy
OBJDUMP = riscv64-elf-objdump
CPPFILT = c++filt

CARGO_ARGS =

MODE ?= release

KERNEL_ELF = target/riscv64gc-unknown-none-elf/$(MODE)/kernel



ifeq ($(MODE), release)
	CARGO_ARGS += --release
endif

all: build dump

build:
	@$(CARGO) build $(CARGO_ARGS) >/dev/null 2>/dev/null
	@$(OBJCOPY) --strip-all '$(KERNEL_ELF)' -O binary '$(testspace)/os.bin'

dump: build
	@$(OBJDUMP) -D '$(KERNEL_ELF)' | $(CPPFILT) > '$(testspace)/os.dump'

run: build
	@qemu-system-riscv64 -nographic -machine virt -m 128M -kernel '$(testspace)/os.bin'

debug: build
	@qemu-system-riscv64 -nographic -machine virt -m 128M -kernel '$(testspace)/os.bin' -s -S

gdb:
	@riscv64-elf-gdb -ex '$(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'
