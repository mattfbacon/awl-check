use std::collections::{BTreeMap, BTreeSet};
use std::io::Read as _;

use unicode_segmentation::UnicodeSegmentation as _;

use crate::awl::{load_awl, Sublist as AwlSublist};
use crate::lemmas::Lemmas;

mod awl;
mod lemmas;

fn main() {
	let raw_lemmas = include_str!("../lemma.txt");
	let lemmas = Lemmas::load(raw_lemmas);

	let raw_awl = include_str!("../awl.txt");
	let awl = load_awl(&lemmas, raw_awl);

	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();
	input.make_ascii_lowercase();
	let words = input.unicode_word_indices();

	let mut counts = BTreeMap::<AwlSublist, BTreeMap<&str, BTreeSet<&str>>>::new();

	for (word_pos, actual_word) in words {
		let base_word = lemmas.resolve(actual_word);

		if let Some((awl_list, awl_word)) = awl.get(base_word).copied() {
			counts
				.entry(awl_list)
				.or_default()
				.entry(awl_word)
				.or_default()
				.insert(actual_word);

			print!(
				"{start}..{end} {actual_word:?} is in AWL sublist {awl_list}",
				start = word_pos,
				end = word_pos + actual_word.len(),
			);

			if actual_word != awl_word {
				print!(" (note it is in the AWL as {awl_word:?})");
			}

			println!();
		}
	}

	println!();
	for (sublist, words) in &counts {
		print!(
			"You have {count} word{s} from Sublist {sublist}",
			s = if words.len() == 1 { " " } else { "s" },
			count = words.len(),
		);

		let mut words = words.iter().map(|(_awl_word, actual_set)| {
			// We only care that we get one of the words that was actually in the essay.
			// That is, if the word is in the essay in multiple forms, it only counts for one AWL word, but we want to list the form of the word that was actually used, so we just pick one.
			actual_set.first().unwrap()
		});
		if let Some(first) = words.next() {
			print!(": {first}");
			for rest in words {
				print!(", {rest}");
			}
		}
		println!();
	}
}
