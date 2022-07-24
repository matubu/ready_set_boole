use std::iter::Rev;
use std::str::Chars;

enum Node {
	enum Leave {
		Not(char),
		Yes(char)
	},
	enum Branch {
		left: Option<Node>,
		right: Option<Node>,
		enum op {
			Or,
			And
		}
	}
}

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

	let mut formula = fixup(&mut formula.chars().rev(), false);

	// Karnaugh map
	// fn kmap() {
	// }

	fn	every_conjunction_at_end(formula: &String) -> bool {
		!formula.trim_end_matches('&').contains("&")
	}

	// Expand
	//    [[ab&][cd&]|]
	// -> [[ac|][[ad|][[bc|][bd|]&]&]&]
	// Join
	//    [[ab&]c&]
	// -> [a[bc&]&]
	// Rotation
	//    [[ab&](c)|]
	//    [(c)[ab&]|]
	// -> [[a(c)|][b(c)|]&]

	[ab&[cd&de&&]&]

	fn	refactor(formula: &String) -> String {
		let mut left_most_conjunction = formula.find('&').unwrap();
		// ab&cd||
		//   ^   ^
		let mut parent = left_most_conjunction;
		let mut count: usize = 0;

		loop {
			parent += 1;
			match formula.as_bytes()[parent] as char {
				c @ ('&' | '|') => {
					if parent == left_most_conjunction + 1 && c == '&' {
						// abcd&&|
						//     ^^
						// Cant do anything
						// Move left_most_conjunction "pointer"
						// abcd&&|
						//      ^
						left_most_conjunction = parent;
						continue ;
					}
					if count < 2 {
						break ;
					}
					count -= 2;
				},
				'A'..='Z' => count += 1,
				_ => ()
			}
		};
		println!("{} {}", left_most_conjunction, parent);
		"".to_string()
	}

	while !every_conjunction_at_end(&formula) {
		formula = refactor(&formula);
	}

	formula
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