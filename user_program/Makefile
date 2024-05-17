sources = $(shell find src)
elfS = $(KernelSrcDir)/elf.gen.S
elfrs = $(KernelSrcDir)/elf_funcs.rs.gen

all: $(elfS)

$(elfS): $(sources)
	@cargo build $(CARGO_ARGS)
	@cargo read-manifest | python gen_asm.py $(CARGO_COPYOUT_DIR) $(elfS) $(elfrs)
