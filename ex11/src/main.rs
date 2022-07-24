fn	map(x: u16, y: u16) -> f64 {
	((x as u32) | ((y as u32) << 16)) as f64 / u32::MAX as f64
}

fn	reverse_map(n: f64) -> (u16, u16) {
	(((n * u32::MAX as f64) as u64 & 0xFFFF) as u16, ((n * u32::MAX as f64) as u64 >> 16) as u16)
}

fn	test(x: u16, y: u16) {
	let encoded = map(x, y);
	let decoded = reverse_map(encoded);

	println!("{} {} -> {:?}", x, y, decoded);
	if decoded.0 == x && decoded.1 == y {
		println!("\x1B[92mOK\x1B[0m");
	} else {
		println!("\x1B[91mKO\x1B[0m");
	}
}

fn	main() {
	test(65535, 65535);
	test(65534, 65534);
	test(12, 7);
	test(68, 357);
	test(786, 453);
}