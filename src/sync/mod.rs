use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
pub trait Mutex {
	fn lock(&self);
	fn unlock(&self);
}

pub struct LockGuard<T: Mutex> {
	mutex: *const T,
}
impl<T: Mutex> LockGuard<T> {
	pub fn new(mutex: &T) -> Self {
		mutex.lock();
		Self { mutex }
	}
}
impl<T: Mutex> Drop for LockGuard<T> {
	fn drop(&mut self) {
		unsafe {
			(*self.mutex).unlock();
		}
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
