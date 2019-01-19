use std::boxed::Box;
use std::collections::HashMap;
use std::io::BufRead;

pub enum ScoringMethod {
	Standard,
	Challenge,
}

pub struct Dict {
	sections: HashMap<char, Node>,
	scoring_method: ScoringMethod,
}

type SubNode = Option<Box<Node>>;

struct Node {
	char: char,
	dict: HashMap<char, SubNode>,
}

impl Dict {
	pub fn new(words: Vec<String>, scoring: ScoringMethod) -> Dict {
		unimplemented!();
	}

	pub fn valid_word(self, word: String) -> bool {
		unimplemented!();
	}

	pub fn longest(self, selection: String, word: String) -> String {
		unimplemented!();
	}

	pub fn highest(self, selection: String, word: String) -> String {
		unimplemented!();
	}

	fn set_scoring(mut self, method: ScoringMethod) {
		self.scoring_method = method;
	}

	fn contains(self, word: String) -> bool {
		unimplemented!();
	}

	fn add_word(mut root: Node, word: String) {
		if word.len() > 0 {
			let word_chars = word.as_str().chars().next().unwrap();
		}

		unimplemented!();
	}

	fn add_chars_to_node(mut root: Node, chars: Vec<char>) {
		let mut current = root;
	}
}

pub fn valid_word_from_selection(selection: String, word: String) -> bool {
	unimplemented!();
}

fn standard_score(word: String) -> u16 {
	return word.chars().map(|c| points(c)).sum();
}

fn challenge_scoring(word: String) -> u16 {
	return word
		.chars()
		.enumerate()
		.map(|(i, c)| (i + 1) as u16 * points(c))
		.sum();
}

fn points(letter: char) -> u16 {
	return match letter {
		'e' | 'a' | 'i' | 'o' | 'n' | 'r' | 't' | 'l' | 's' | 'u' => 1,
		'd' | 'g' => 2,
		'b' | 'c' | 'm' | 'p' => 3,
		'f' | 'h' | 'v' | 'w' | 'y' => 4,
		'k' => 5,
		'j' | 'x' => 8,
		'q' | 'z' => 10,
		_ => 0,
	};
}

mod tests {
	use super::*;

	#[test]
	fn standard_score_calculates_correctly() {
		let expected_score = 13;
		let word = String::from("hack");
		let result = standard_score(word);

		assert_eq!(expected_score, result);
	}

	#[test]
	fn standard_score_with_blanks_calculates_correctly() {
		let expected_score = 12;
		let word = String::from("h?ck");
		let result = standard_score(word);

		assert_eq!(expected_score, result);
	}

	#[test]
	fn challenge_scoring_calculates_correctly() {
		let expected = 35;
		let word = String::from("hack");
		let result = challenge_scoring(word);

		assert_eq!(expected, result);
	}

	#[test]
	fn challenge_scoring_with_blank_calculates_correctly() {
		let expected = 33;
		let word = String::from("h?ck");
		let result = challenge_scoring(word);

		assert_eq!(expected, result);
	}
}
