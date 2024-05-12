use std::collections::HashSet;
use itertools::Itertools;
use trie_rs::{Trie, TrieBuilder};

#[derive(Clone)]
pub struct WordList(HashSet<String>);

impl WordList {
    pub fn load_from_file(path: &std::path::Path) -> std::io::Result<Self> {
        Ok(WordList(
            std::fs::read_to_string(path)?
                .lines()
                .map(ToString::to_string)
                .collect()
        ))
    }

    pub fn from_vec(words: Vec<String>) -> Self {
        Self(words.into_iter().collect())
    }

    pub fn words(&self) -> &HashSet<String> {
        &self.0
    }

    pub fn validate_words(&mut self) {
        self.0
            .retain(|word| (3..=14).contains(&word.len()) && word.chars().all(|c| c.is_ascii_alphabetic()));
        self.0 = self.0
            .iter()
            .map(|w| w.to_ascii_lowercase())
            .collect();
    }

    pub fn into_trie(self) -> Trie<u8> {
        let mut builder = TrieBuilder::new();
        self.0.into_iter()
            .sorted()
            .for_each(|w| builder.push(w));
        builder.build()
    }

    pub fn retain_only_buildable(&mut self, letters: &Vec<char>) {
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
    }
}