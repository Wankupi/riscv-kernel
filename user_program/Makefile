sources = $(shell find src)
elfS = $(KernelSrcDir)/user/elf.gen.S
elfrs = $(KernelSrcDir)/user/elf_funcs_gen.rs

all: $(elfS)

$(elfS): $(sources) gen_asm.py Cargo.toml
	@cargo build $(CARGO_ARGS)
	@cargo read-manifest | python gen_asm.py $(CARGO_COPYOUT_DIR) $(elfS) $(elfrs)

clean:
	rm -f $(elfS) $(elfrs)
