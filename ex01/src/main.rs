fn	adder(a: u32, b: u32) -> u32 {
	let	mut carry: bool = false;
	let mut result: u32 = a ^ b;

	for i in 0..32 {
		result ^= (carry as u32) << i;
		carry = (1
			<< ((a >> i) & 1)
			<< ((b >> i) & 1)
			<< (carry as u32)
		) > 2;
	}

	result
}

fn	multiplier(a: u32, b: u32) -> u32 {
	let mut result: u32 = 0;

	for i in 0..32 {
		if ((b >> i) & 1) == 1 {
			result = adder(result, a << i);
		}
	}

	result
}

fn	test(a: u32, b: u32) {
	println!("{} == {}", a * b, multiplier(a, b));
}

fn	main() {
	test(0, 0);
	test(1, 1);
	test(0, 1);
	test(5, 5);
	test(9874, 12896);
	test(75, 35);
	test(32, 32);
}