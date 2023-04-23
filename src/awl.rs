use std::collections::HashMap;

use crate::lemmas::Lemmas;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sublist(pub u8);

impl std::fmt::Display for Sublist {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(formatter)
	}
}

pub type Awl<'a> = HashMap<&'a str, (Sublist, &'a str)>;

pub fn load_awl<'a>(lemmas: &Lemmas<'a>, raw: &'a str) -> Awl<'a> {
	let mut mapping = Awl::new();

	let mut lines = raw.lines();
	while let Some(header) = lines.next() {
		let header = header.trim().strip_prefix("Sublist ").unwrap();
		let list_num = header.parse().unwrap();

		let empty = lines.next().unwrap().trim();
		debug_assert!(empty.is_empty());

		let words = lines.next().unwrap().trim();
		let words = words
			.split('•')
			.map(str::trim)
			.filter(|word| !word.is_empty())
			.map(|actual_word| (actual_word, lemmas.resolve(actual_word)));
		mapping.extend(words.map(|(base_word, awl_word)| (base_word, (Sublist(list_num), awl_word))));
	}

	mapping
}
