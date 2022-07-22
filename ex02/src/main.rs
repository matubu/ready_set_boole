fn	gray_code(n: u32) -> u32 {
	n ^ (n >> 1)
}

fn	test(n: u32, expected: u32) {
	println!("{} == {}", expected, gray_code(n));
}

fn	main() {
	test(0, 0);
	test(1, 1);
	test(2, 3);
	test(3, 2);
	test(4, 6);
	test(5, 7);
	test(6, 5);
	test(7, 4);
	test(8, 12);
}