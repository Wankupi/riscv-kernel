PROJECT_DIR = $(shell pwd)
BUILD_DIR = build
SRC_DIR = src

# compile setting
CARGO_MODE ?= release

# run setting
QEMU_MEMORY_SIZE = 128M
QEMU_SET_BIOS = false
QEMU_SET_BIOS_CONFIG = /home/wkp/codes/riscv-sbi/build/sbi.bin
QEMU_SET_SMP = false
QEMU_SET_SMP_CONFIG = 1

# toolchain
CARGO = cargo
OBJCOPY = riscv64-linux-gnu-objcopy
OBJDUMP = riscv64-linux-gnu-objdump
# CC = riscv64-linux-gnu-gcc
CC = riscv64-elf-gcc
CPPFILT = c++filt
LD = riscv64-linux-gnu-ld
# LD = riscv64-linux-gnu-ld

CARGO_DIST_DIR = $(PROJECT_DIR)/$(BUILD_DIR)/rust
CARGO_COPYOUT_DIR = $(PROJECT_DIR)/$(BUILD_DIR)
CARGO_ARGS = --target-dir '$(CARGO_DIST_DIR)' -Z unstable-options --out-dir '$(CARGO_COPYOUT_DIR)'
LIB_KERNEL = $(CARGO_DIST_DIR)/riscv64gc-unknown-none-elf/$(CARGO_MODE)/libkernel.a

QEMU_RUN_ARGS = -nographic -machine virt -m $(QEMU_MEMORY_SIZE)

ifeq ($(QEMU_SET_BIOS),true)
	QEMU_RUN_ARGS += -bios $(QEMU_SET_BIOS_CONFIG)
endif
ifneq ($(QEMU_SET_SMP),)
	QEMU_RUN_ARGS += -smp $(QEMU_SET_SMP_CONFIG)
endif

# QEMU_RUN_ARGS += -device loader,addr=0x80000000,file=/home/wkp/codes/kernel/test/1/a.out
# QEMU_RUN_ARGS += -mem-path 0x100000000:/home/wkp/codes/kernel/test/1/a.out

ifeq ($(CARGO_MODE), release)
	CARGO_ARGS += --release
endif

# find source files
RUST_FILES = $(shell find $(SRC_DIR) -name '*.rs')
ASM_FILES = $(shell find $(SRC_DIR) -name '*.S')
C_FILES = $(shell find $(SRC_DIR) -name '*.c')

# dist files
OS_ELF = $(BUILD_DIR)/os.elf
OS_BIN = $(BUILD_DIR)/os.bin
OS_DUMP = $(BUILD_DIR)/os.dump
ASM_TARGETS = $(ASM_FILES:%.S=$(BUILD_DIR)/%.o)
C_TARGETS = $(C_FILES:%.c=$(BUILD_DIR)/%.o)


all: user $(OS_BIN) dump

user:
	make -C user_program

COMMON_FLAGS = -march=rv64gc -mabi=lp64d -mcmodel=medany -ffreestanding -nostdlib -nostartfiles -Wall -O2
LINK_CONFIG = -O2 -pie --emit-relocs

$(ASM_TARGETS): $(BUILD_DIR)/%.o: %.S
	@mkdir -p $(dir $@)
	@$(CC) -c $< -o $@ -g -I $(SRC_DIR) ${COMMON_FLAGS}

$(C_TARGETS): $(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	@$(CC) -c $< -o $@ -g $(COMMON_FLAGS)

$(LIB_KERNEL): user $(RUST_FILES) .cargo/config.toml Cargo.toml
	@$(CARGO) build $(CARGO_ARGS)

$(OS_ELF): $(SRC_DIR)/linker.ld $(ASM_TARGETS) $(LIB_KERNEL) $(C_TARGETS)
	@mkdir -p $(BUILD_DIR)
	@$(LD) $(LINK_CONFIG) -T $^ -o $@

$(OS_BIN): $(OS_ELF)
	@$(OBJCOPY) --strip-all $< -O binary $@

$(OS_DUMP): $(OS_ELF)
	@$(OBJDUMP) -D $< | $(CPPFILT) > $@

dump: $(OS_DUMP)



run: $(OS_BIN) dump
	@qemu-system-riscv64 $(QEMU_RUN_ARGS) -kernel $<

debug: $(OS_BIN) dump
	@qemu-system-riscv64 $(QEMU_RUN_ARGS) -kernel $< -s -S


GDB := riscv64-elf-gdb

gdb: dump
	@$(GDB) -ex 'target remote localhost:1234' -x '.gdbinit'

clean:
	@make -C user_program clean
	@rm -rf $(BUILD_DIR)

.PHONY: all clean dump run debug gdb user

KernelSrcDir = $(PROJECT_DIR)/$(SRC_DIR)
export BUILD_DIR CC CARGO_ARGS CARGO_COPYOUT_DIR KernelSrcDir

ram:
	make -C test
