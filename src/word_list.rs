use std::collections::HashSet;

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

    pub fn retain_only_possible(&mut self, letters: &Vec<char>) {
        self.0
            .retain(|word| word.chars().all(|c| letters.contains(&c)));
    }
}