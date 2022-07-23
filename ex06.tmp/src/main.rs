use std::iter::Rev;
use std::str::Chars;

fn conjunctive_normal_form(formula: &str) -> String {
	fn	fixup(it: &mut Rev<Chars>, neg: bool) -> String {
		let and = if neg { "|" } else { "&" };
		let or = if neg { "&" } else { "|" };

		match it.next() {
			Some('!') => fixup(it, !neg),
			Some('&') => format!("{1}{0}{and}", fixup(it, neg), fixup(it, neg)),
			Some('|') => format!("{1}{0}{or}", fixup(it, neg), fixup(it, neg)),
			Some('^') => format!("{1}{0}{and}", fixup(it, neg), fixup(it, !neg)),
			Some('>') => format!("{1}{0}{or}", fixup(it, neg), fixup(it, !neg)),
			Some('=') => {
				let copy_it = &mut (*it).clone();
				format!("{1}{0}{and}{2}{3}{and}{or}",
					fixup(it, neg), fixup(it, neg),
					fixup(copy_it, !neg), fixup(copy_it, !neg)
				)
			},
			Some(c @ 'A'..='Z') => format!("{}{op}", c, op=if neg { "!" } else { "" }),
			None => panic!("\x1B[91merror\x1B[0m end of string"),
			_ => panic!("\x1B[91merror\x1B[0m invalid char")
		}
	}

	fixup(&mut formula.chars().rev(), false)

	//    [[ab|]c|]
	// -> [c[ab|]|]
	//    [[ab|][cd|]|]
	// -> [[ab&][cd&]|] ???
	fn refactor()
}

fn	test(formula: &str, expected: &str) {
	println!("{} => {} == {}", formula, conjunctive_normal_form(formula), expected);
}

fn	main() {
	test("AB&!", "A!B!|");
	test("AB|!", "A!B!&");
	test("AB|C&", "AB|C&");
	test("AB|C|D|", "ABCD|||");
	test("AB&C&D&", "ABCD&&&");
	test("AB&!C!|", "A!B!C!||");
	test("AB&!C!|", "A!B!C!||");
}