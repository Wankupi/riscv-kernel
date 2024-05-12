extern "C" {
	pub fn _trap_entry();
	pub fn stext();
	pub fn etext();
	pub fn srodata();
	pub fn erodata();
	pub fn sdata();
	pub fn edata();
	pub fn sbss();
	pub fn ebss();
	pub fn skernel();
	pub fn ekernel();
	pub fn _user_ret(satp: usize) -> !;
	pub fn boot_stack_top();
}


extern "C" {
	pub fn elf1_start();
	pub fn elf1_end();
}