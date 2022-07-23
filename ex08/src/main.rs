fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
	let mut powerset = Vec::<Vec<i32>>::new();

	let permutations = 1 << set.len();
	for i in 0..permutations {
		let mut j = 0;
		let mut permutation = Vec::<i32>::new();
		for value in set {
			if (i >> j) % 2 == 1 {
				permutation.push(*value);
			}
			j += 1;
		}
		powerset.push(permutation);
	}
	
	powerset
}

fn	test(set: &[i32]) {
	println!("{:?} => {:?}", set, powerset(set));
}

fn	main() {
	test(&[1]);
	test(&[1, 2]);
	test(&[1, 2, 3]);
	test(&[1, 2, 3, 4]);
}