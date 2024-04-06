use core::ops;
pub struct UnsafeArray<T> {
	data: *mut T,
}
impl<T> UnsafeArray<T> {
	pub const fn new() -> Self {
		Self { data: 0 as *mut T }
	}
	pub fn init(&mut self, data: *mut T) {
		self.data = data;
	}
	pub fn get(&self, index: usize) -> &T {
		unsafe { &*self.data.offset(index as isize) }
	}
	pub fn get_mut(&self, index: usize) -> &mut T {
		unsafe { &mut *self.data.offset(index as isize) }
	}
}
impl<T> ops::Index<usize> for UnsafeArray<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		self.get(index)
	}
}
impl<T> ops::IndexMut<usize> for UnsafeArray<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.get_mut(index)
	}
}