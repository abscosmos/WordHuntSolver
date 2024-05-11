use std::collections::HashSet;

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
        Self(HashSet::from_iter(words.into_iter()))
    }

    pub fn words(&self) -> &HashSet<String> {
        &self.0
    }

    pub fn words_mut(&mut self) -> &mut HashSet<String> {
        &mut self.0
    }

    // pub fn validate_words(&mut self) {
    //     self.0
    //         .retain(|word| (3..=14).contains(&word.len()) && word.chars().all(|c| c.is_ascii_alphabetic()));
    //     self.0.iter()
    //         .for_each(|s| { *s = s.to_ascii_lowercase(); });
    // }

    pub fn retain_only_possible(&mut self, letters: &Vec<char>) {
        self.0
            .retain(|word| word.chars().all(|c| letters.contains(&c)));
    }
}