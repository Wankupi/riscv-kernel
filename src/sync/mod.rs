use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
pub trait Mutex {
	fn lock(&self);
	fn unlock(&self);
}

pub struct LockGuard<'a, T> {
	mutex: &'a T,
}

impl<'a, T: Mutex> LockGuard<'a, T> {
	pub fn new(mutex: &'a T) -> Self {
		mutex.lock();
		Self { mutex }
	}
	pub fn drop(&mut self) {
		self.mutex.unlock();
	}
}

pub struct SpinLock {
	lock: AtomicUsize,
}
impl SpinLock {
	pub const fn new() -> Self {
		Self {
			lock: AtomicUsize::new(0),
		}
	}
	pub fn init(&self) {
		self.lock.store(0, Relaxed);
	}
}
impl Mutex for SpinLock {
	fn lock(&self) {
		loop {
			match self.lock.compare_exchange(0, 1, Relaxed, Relaxed) {
				Ok(_) => break,
				Err(_) => continue,
			}
		}
	}
	fn unlock(&self) {
		self.lock.store(0, Relaxed);
	}
}
