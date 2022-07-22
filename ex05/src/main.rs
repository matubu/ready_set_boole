use std::iter::Rev;
use std::str::Chars;

fn	negation_it(it: &mut Rev<Chars>, negate: bool) -> String {
	match it.next() {
		Some('!') => format!("{}", negation_it(it, !negate)),
		Some('&') => format!("{1}{0}{op}", negation_it(it, negate), negation_it(it, negate),
			op=if negate { "|" } else { "&" }),
		Some('|') => format!("{1}{0}{op}", negation_it(it, negate), negation_it(it, negate),
			op=if negate { "&" } else { "|" }),
		Some('^') => format!("{1}{0}{op}", negation_it(it, negate), negation_it(it, !negate),
			op=if negate { "|" } else { "&" }),
		Some('>') => format!("{1}{0}{op}", negation_it(it, negate), negation_it(it, !negate),
			op=if negate { "&" } else { "|" }),
		Some('=') => {
			let copy = &mut (*it).clone();
			format!("{1}{0}{and}{2}{3}{and}{or}",
				negation_it(it, negate), negation_it(it, negate),
				negation_it(copy, !negate), negation_it(copy, !negate),
				and=if negate { "|" } else { "&" },
				or=if negate { "&" } else { "|" }
			)
		},
		Some(c @ 'A'..='Z') => format!("{}{op}", c, op=if negate { "!" } else { "" }),
		None => panic!("\x1B[91merror\x1B[0m end of string"),
		_ => panic!("\x1B[91merror\x1B[0m invalid char")
	}
}

fn negation_normal_form(formula: &str) -> String {
	negation_it(&mut formula.chars().rev(), false)
}

fn	test(formula: &str, expected: &str) {
	println!("{} => {} == {}", formula, negation_normal_form(formula), expected);
}

fn	main() {
	test("AB&!", "A!B!|");
	test("AB|!", "A!B!&");
	test("AB>", "A!B|");
	test("AB=", "AB&A!B!&|");
	test("AB=!", "A!B!|BA|&");
	test("AB|C&!", "A!B!&C!|");
}