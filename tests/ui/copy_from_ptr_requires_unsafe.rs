use aligned_vec::AVec;

fn main() {
	let _ = AVec::<u8>::__copy_from_ptr(1, core::ptr::null::<u8>(), 1);
}
