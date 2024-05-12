pub const uart_base_addr: usize = 0x10000000;
pub const system_reset_addr: usize = 0x100000;

pub const PAGE_SIZE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
pub const PA_WIDTH: usize = 56;
pub const PA_SIZE: usize = 1 << PA_WIDTH;
pub const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;
pub const PPN_SIZE: usize = 1 << PPN_WIDTH;
pub const VT_MAP_SIZE: usize = 4096 / 8;
