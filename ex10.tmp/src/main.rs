fn	map(x: u16, y: u16) -> f64 {
	((x as u32) | ((y as u32) << 16)) as f64 / u32::MAX as f64
}

fn	reverse_map(n: f64) -> (u16, u16) {
	(((n * u32::MAX as f64) as u32 & 0xFF) as u16, ((n * u32::MAX as f64) as u32 >> 16) as u16)
}

fn	main() {
	let encoded = map(7878, 453);
	let decoded = reverse_map(encoded);
	
	println!("{} {:?}", encoded, decoded);
}