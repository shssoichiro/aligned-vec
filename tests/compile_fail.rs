#[test]
fn copy_from_ptr_requires_unsafe() {
	let tests = trybuild::TestCases::new();
	tests.compile_fail("tests/ui/copy_from_ptr_requires_unsafe.rs");
}
