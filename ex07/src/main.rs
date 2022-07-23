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

fn sat(formula: &str) -> bool {
	let mut vars = collect_vars(formula);

	let permutations = 1 << vars.len();
	for i in 0..permutations {
		let mut j = vars.len();
		for (_, value) in &mut vars {
			j -= 1;
			*value = (i >> j) % 2 == 1;
		}
		if eval_formula(formula, &vars) {
			return true;
		}
	}
	
	false
}

fn	test(formula: &str, expected: bool) {
	println!("{} => {} == {}", formula, sat(formula), expected);
}

fn	main() {
	test("AB|", true);
	test("AB&", true);
	test("AA!&", false);
	test("AA^", false);
}