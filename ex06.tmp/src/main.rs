use std::iter::Rev;
use std::str::Chars;

fn conjunctive_normal_form(formula: &str) -> String {
	fn	get_token(it: &mut Rev<Chars>) -> String {
		match it.next() {
			Some('!') => format!("{}!", get_token(it)),
			Some('&') => format!("{}{}&", get_token(it), get_token(it)),
			Some('|') => format!("{}{}|", get_token(it), get_token(it)),
			Some(c @ 'A'..='Z') => c.to_string(),
			_ => "".to_string()
		}
	}

	fn	fixup(it: &mut Rev<Chars>, neg: bool) -> String {
		fn	fixup_or(a: String, b: String) -> String {
			fn fixup_split_and(s: String) -> (String, Vec<String>) {
				let first_and = s.find("&").unwrap();
				let ands = &s[first_and..];
				let tokens_it = &mut s[..first_and].chars().rev();
				let mut tokens = Vec::<String>::new();

				for _ in 0..ands.len()+1 {
					let token = get_token(tokens_it);
					tokens.push(token);
				}

				(ands.to_string(), tokens)
			}

			// AB|   CDE&&   |
			// Move all & to then end
			// Apply | to every token from b
			// AB|C|   AB|D|   AB|E|   &&
			fn	fixup_or_single_side(a: String, b: String) -> String {
				let mut tmp = String::new();

				let (ands, tokens) = fixup_split_and(b);

				for token in tokens {
					tmp += &format!("{}{}|", token, a);
				}

				format!("{tmp}{ands}")
			}

			fn	fixup_or_double_side(a: String, b: String) -> String {
				let mut tmp = String::new();

				let (ands_a, tokens_a) = fixup_split_and(a);
				let (ands_b, tokens_b) = fixup_split_and(b);

				for token_a in &tokens_a {
					for token_b in &tokens_b {
						tmp += &format!("{}{}|", token_a, token_b);
					}
				}

				format!("{tmp}{ands_a}{ands_b}")
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
				(true, true) => fixup_or_double_side(a, b),
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
	test("AB&CD&|", "");
	test("ABC&&DEF&&|", "");
}