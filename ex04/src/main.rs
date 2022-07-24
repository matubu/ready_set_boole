use std::iter::Rev;
use std::str::Chars;
use std::collections::BTreeMap;

fn	eval_formula_it(it: &mut Rev<Chars>, vars: &BTreeMap::<char, bool>) -> bool {
	match it.next() {
		Some('!') => !eval_formula_it(it, vars),
		Some('&') => eval_formula_it(it, vars) & eval_formula_it(it, vars),
		Some('|') => eval_formula_it(it, vars) | eval_formula_it(it, vars),
		Some('^') => eval_formula_it(it, vars) ^ eval_formula_it(it, vars),
		Some('>') => eval_formula_it(it, vars) <= eval_formula_it(it, vars),
		Some('=') => eval_formula_it(it, vars) == eval_formula_it(it, vars),
		Some(c @ 'A'..='Z') => *vars.get(&c).unwrap(),
		None => panic!("\x1B[91merror\x1B[0m end of string"),
		_ => panic!("\x1B[91merror\x1B[0m invalid char")
	}
}

fn	eval_formula(formula: &str, vars: &BTreeMap::<char, bool>) -> bool {
	eval_formula_it(&mut formula.chars().rev(), vars)
}

fn	collect_vars(formula: &str) -> BTreeMap::<char, bool> {
	let mut vars = BTreeMap::<char, bool>::new();

	for c in formula.chars() {
		if c >= 'A' && c <= 'Z' {
			vars.insert(c, false);
		}
	}

	vars
}

fn	print_truth_table(formula: &str) {
	let mut vars = collect_vars(formula);

	for (var, _) in &vars {
		print!("| {var} ");
	}
	println!("| = |");

	for _ in 0..vars.len() {
		print!("|---");
	}
	println!("|---|");

	let permutations = 1 << vars.len();
	for i in 0..permutations {
		let mut j = vars.len();
		for (_, value) in &mut vars {
			j -= 1;
			*value = (i >> j) % 2 == 1;
			print!("| {} ", *value as u8);
		}
		println!("| {} |", eval_formula(formula, &vars) as u8);
	}
}

fn	main() {
	print_truth_table("A");
	print_truth_table("AB&C|");
	print_truth_table("AB&C|D|");

	print_truth_table("AB&CD&|");
	print_truth_table("A!B!|C!D!|&");
}