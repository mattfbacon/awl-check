use std::collections::HashMap;

pub struct Lemmas<'a>(HashMap<&'a str, &'a str>);

impl<'src> Lemmas<'src> {
	pub fn load(raw: &'src str) -> Self {
		let mut ret = Self(HashMap::new());

		for line in raw.lines() {
			let line = line.trim();
			if line.starts_with(';') {
				continue;
			}
			let (lemma_freq, syns) = line.split_once(" -> ").unwrap();
			let (lemma, _freq) = lemma_freq.split_once('/').unwrap_or((lemma_freq, ""));
			let syns = syns.split(',');
			for syn in syns {
				ret.0.entry(syn).or_insert(lemma);
			}
		}

		ret
	}

	pub fn resolve<'a>(&self, actual_word: &'a str) -> &'a str
	where
		'src: 'a,
	{
		debug_assert!(
			actual_word.chars().all(|ch| !ch.is_ascii_uppercase()),
			"{actual_word:?} is not lowercase!"
		);
		self.0.get(actual_word).copied().unwrap_or(actual_word)
	}
}
