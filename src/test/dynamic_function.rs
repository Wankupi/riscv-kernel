use crate::print::printk;

trait TestDyn {
	fn test_dyn(&self) -> usize;
}
struct A {}
struct B {}
impl TestDyn for A {
	fn test_dyn(&self) -> usize {
		1
	}
}
impl TestDyn for B {
	fn test_dyn(&self) -> usize {
		2
	}
}


fn output_test(x: &dyn TestDyn) {
	let v = x.test_dyn();
	if v == 1 {
		printk(b"res = 1\n");
	}
	else {
		printk(b"res != 1\n");
	}
}

#[no_mangle]
pub extern "C" fn test_dynamic_function() {
	let a = A{};
	let b = B{};
	if a.test_dyn() == 1 && b.test_dyn() == 2 {
		printk(b"test_dynamic_function: pass\n");
	}
	else {
		printk(b"test_dynamic_function: fail\n");
	}
}
