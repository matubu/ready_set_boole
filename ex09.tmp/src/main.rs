use std::iter::Rev;
use std::str::Chars;
use std::collections::HashSet;

fn	eval_set_it(it: &mut Rev<Chars>, sets: &Vec<Vec<i32>>) -> HashSet<i32> {
	match it.next() {
		Some('!') => {
			eval_set_it(it, sets);
			HashSet::new()
		},
		Some('&') => {
			eval_set_it(it, sets).intersection(&eval_set_it(it, sets)).copied().collect()
		},
		Some('|') => {
			eval_set_it(it, sets).union(&eval_set_it(it, sets)).copied().collect()
		},
		Some('^') => {
			let a = eval_set_it(it, sets);
			let b = eval_set_it(it, sets);
			a.union(&b).copied().collect::<HashSet<i32>>()
				.difference(&a.intersection(&b).copied().collect()).copied().collect()
		},
		// Some('>') => eval_set_it(it, sets) <= eval_set_it(it, sets),
		Some('=') => {
			let a = eval_set_it(it, sets);
			let b = eval_set_it(it, sets);
			if a == b { a } else { HashSet::new() }
		},
		Some(c @ 'A'..='Z') => sets[c as usize - 'A' as usize].iter().copied().collect(),
		None => panic!("\x1B[91merror\x1B[0m end of string"),
		_ => panic!("\x1B[91merror\x1B[0m invalid char")
	}
}

fn	eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
	eval_set_it(&mut formula.chars().rev(), sets).into_iter().collect()
}

fn	test(formula: &str, sets: &Vec<Vec<i32>>) {
	println!("{} => {:?}", formula, eval_set(formula, sets));
}

fn	main() {
	test("AB&", &vec![
		vec![0, 1, 2],
		vec![0, 3, 4],
	]);
	// [0]

	test("AB|", &vec![
		vec![0, 1, 2],
		vec![3, 4, 5],
	]);
	// [0, 1, 2, 3, 4, 5]

	test("AB=", &vec![
		vec![0, 1, 2],
		vec![0, 2, 1],
	]);

	test("AB^", &vec![
		vec![0, 1, 2, 7],
		vec![0, 2, 1, 8],
	]);

	test("A!", &vec![
		vec![0, 1, 2],
	]);
	// []
	
	// TODO fix ['!', '>', '=']

	// BA!&
	// Remove all A from B ?
}