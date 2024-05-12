use std::collections::HashSet;
use itertools::Itertools;
use trie_rs::{Trie, TrieBuilder};

#[derive(Clone)]
pub struct WordListBuilder(HashSet<String>);

impl WordListBuilder {
    pub fn from_words<T>(iterable: T) -> Self where T: IntoIterator, T::Item: Into<String> {
        Self(iterable.into_iter().map(Into::into).collect())
    }

    pub fn validate(mut self) -> Self {
        self.0
            .retain(|word| (3..=14).contains(&word.len()) && word.chars().all(|c| c.is_ascii_alphabetic()));
        self.0 = self.0
            .iter()
            .map(|w| w.to_ascii_lowercase())
            .collect();

        self
    }

    pub fn build(&self) -> WordList {
        let mut builder = TrieBuilder::new();
        self.0.iter()
            .sorted()
            .for_each(|w| builder.push(w));
        WordList(builder.build())
    }

    pub fn only_using_letters(mut self, letters: &Vec<char>) -> Self {
        let mut unique = letters.iter().unique().collect::<String>();

        self.0
            .retain(|word| word.chars().all(|c| unique.contains(c)));

        let mut counts = [0; 26];

        letters.iter()
            .for_each(|&c| { counts[c as usize - b'a' as usize] += 1; });

        self.0.retain(|word| {
            let mut letters = counts;

            for c in word.chars() {
                let idx = c as usize - b'a' as usize;
                if letters[idx] == 0 { return false; }
                letters[idx] -= 1;
            }

            true
        });

        self
    }
}

pub struct WordList(pub Trie<u8>);