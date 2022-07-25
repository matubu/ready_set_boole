use std::iter::Rev;
use std::str::Chars;

fn conjunctive_normal_form(formula: &str) -> String {
	fn	fixup(it: &mut Rev<Chars>, neg: bool) -> String {
		fn	fixup_or(a: String, b: String) -> String {
			// AB|   CDE&&   |
			// Move all & to then end
			// Apply | to every token from b
			// AB|C|   AB|D|   AB|E|   &&
			fn	fixup_or_single_side(a: String, b: String) -> String {
				fn	get_token(it: &mut Rev<Chars>) -> String {
					match it.next() {
						Some('!') => format!("{}!", get_token(it)),
						Some('&') => format!("{}{}&", get_token(it), get_token(it)),
						Some('|') => format!("{}{}|", get_token(it), get_token(it)),
						Some(c @ 'A'..='Z') => c.to_string(),
						_ => "".to_string()
					}
				}

				let mut tmp = String::new();

				let first_and = b.find("&").unwrap();
				let ands = &b[first_and..];

				let tokens = &b[..first_and];

				let tokens_it = &mut tokens.chars().rev();

				for _ in 0..ands.len()+1 {
					let token = get_token(tokens_it);
					tmp += &format!("{}{}|", token, a);
				}

				format!("{tmp}{ands}")
			}

			match (a.ends_with("&"), b.ends_with("&")) {
				// Easy
				(false, false) => format!("{}{}|", b, a),

				// Single side
				//    [[ab&](c)|]
				// or [(c)[ab&]|]
				// -> [[ac|][bc|]&]
				//    [(e)[a[b[cd&]&]&]|]
				// -> [ae|[be|[ce|de|&]&]&]
				(true, false) => fixup_or_single_side(b, a),
				(false, true) => fixup_or_single_side(a, b),

				// Double side
				//    [[ab&][cd&]|]
				// -> [[ac|][[ad|][[bc|][bd|]&]&]&]
				//    [[abc&&][def&&]|]
				// -> ??
				// (true, true) => format!("{1}{0}&", a, b),

				_ => "".to_string()
			}
		}

		fn	fixup_and(a: String, b: String) -> String {
			match (a.ends_with("&"), b.ends_with("&")) {
				// Easy
				(false, true) => format!("{}{}&", a, b),
				(false, false)
				| (true, false) => format!("{}{}&", b, a),

				// Insert
				// [[abc&&][def&&]&]
				// [[def[abc&&]&&]&]
				(true, true) => {
					let mut tmp = a.clone();
					tmp.insert_str(a.find("&").unwrap(), &b);
					format!("{}&", tmp)
				}
			}
		}

		let and = |a, b| if neg { fixup_or(a, b) } else { fixup_and(a, b) };
		let or = |a, b| if neg { fixup_and(a, b) } else { fixup_or(a, b) };

		match it.next() {
			Some('!') => fixup(it, !neg),
			Some('&') => and(fixup(it, neg), fixup(it, neg)),
			Some('|') => or(fixup(it, neg), fixup(it, neg)),
			Some('^') => and(fixup(it, neg), fixup(it, !neg)),
			Some('>') => or(fixup(it, neg), fixup(it, !neg)),
			Some('=') => {
				let copy_it = &mut (*it).clone();
				or(and(fixup(it, neg), fixup(it, neg)),
					and(fixup(copy_it, !neg), fixup(copy_it, !neg)))
			},
			Some(c @ 'A'..='Z') => format!("{}{op}", c, op=if neg { "!" } else { "" }),
			None => panic!("\x1B[91merror\x1B[0m end of string"),
			_ => panic!("\x1B[91merror\x1B[0m invalid char")
		}
	}

	// Karnaugh map
	// fn kmap() {
	// }

	fixup(&mut formula.chars().rev(), false)
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
	test("ABC&&DEF&&&", "DEFABC&&&&&");
	test("AB|CDE&&|", "");
}