BUILD_DIR = build
SRC_DIR = src

CARGO = cargo
OBJCOPY = riscv64-elf-objcopy
OBJDUMP = riscv64-elf-objdump
CC = riscv64-elf-gcc
CPPFILT = c++filt
LD = riscv64-elf-ld

CARGO_DIST_DIR = $(BUILD_DIR)/rust
CARGO_ARGS = --target-dir '$(CARGO_DIST_DIR)' -Z unstable-options --out-dir $(BUILD_DIR)
CARGO_MODE ?= release


LIB_KERNEL = $(CARGO_DIST_DIR)/riscv64gc-unknown-none-elf/$(CARGO_MODE)/libkernel.a

RUST_FILES = $(shell find $(SRC_DIR) -name '*.rs')
ASM_FILES = $(shell find $(SRC_DIR) -name '*.S')
C_FILES = $(shell find $(SRC_DIR) -name '*.c')

OS_ELF = $(BUILD_DIR)/os.elf
OS_BIN = $(BUILD_DIR)/os.bin
OS_DUMP = $(BUILD_DIR)/os.dump

ifeq ($(CARGO_MODE), release)
	CARGO_ARGS += --release
endif

all: $(OS_BIN) dump


ASM_TARGETS = $(ASM_FILES:%.S=$(BUILD_DIR)/%.o)
$(ASM_TARGETS): $(BUILD_DIR)/%.o: %.S
	@mkdir -p $(dir $@)
	@$(CC) -c $< -o $@

C_TARGETS = $(C_FILES:%.c=$(BUILD_DIR)/%.o)
$(C_TARGETS): $(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@$(CC) -c $< -o $@

$(LIB_KERNEL): $(RUST_FILES) .cargo/config.toml Cargo.toml
	@$(CARGO) build $(CARGO_ARGS) 2>/dev/null

$(OS_ELF): $(SRC_DIR)/linker.ld $(ASM_TARGETS) $(LIB_KERNEL) $(C_TARGETS)
	@mkdir -p $(BUILD_DIR)
	@$(LD) -O2 -T $^ -o $@

$(OS_BIN): $(OS_ELF)
	@$(OBJCOPY) --strip-all $< -O binary $@

$(OS_DUMP): $(OS_ELF)
	@$(OBJDUMP) -D $< | $(CPPFILT) > $@

dump: $(OS_DUMP)

QEMU_RUN_ARGS = -nographic -machine virt -m 128M
QEMU_RUN_ARGS += -bios /home/wkp/codes/kernel/build/sbi.bin


run: $(OS_BIN) dump
	@qemu-system-riscv64 $(QEMU_RUN_ARGS) -kernel $<

debug: $(OS_BIN) dump
	@qemu-system-riscv64 $(QEMU_RUN_ARGS) -kernel $< -s -S

gdb: dump
	@riscv64-elf-gdb -ex 'file $(OS_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

clean:
	@rm -r $(BUILD_DIR)

.PHONY: all clean dump run debug gdb
