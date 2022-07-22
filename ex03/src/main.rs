use std::iter::Rev;
use std::str::Chars;

fn	eval_formula_it(it: &mut Rev<Chars>) -> bool {
	match it.next() {
		Some('0') => false,
		Some('1') => true,
		Some('!') => !eval_formula_it(it),
		Some('&') => eval_formula_it(it) & eval_formula_it(it),
		Some('|') => eval_formula_it(it) | eval_formula_it(it),
		Some('^') => eval_formula_it(it) ^ eval_formula_it(it),
		Some('>') => eval_formula_it(it) <= eval_formula_it(it),
		Some('=') => eval_formula_it(it) == eval_formula_it(it),
		_ => false
	}
}

fn	eval_formula(formula: &str) -> bool {
	eval_formula_it(&mut formula.chars().rev())
}

fn	test(formula: &str, expected: bool) {
	println!("{} == {}", expected, eval_formula(formula));
}

fn	main() {
	test("10&", false);
	test("10|", true);
	test("11>", true);
	test("10=", false);
	test("1011||=", true);
	test("10|1&", true);
	test("101|&", true);
}