use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct FoundWord {
    pub word: String,
    pub path: Vec<u8>
}

impl FoundWord {
    pub fn from_slice(word: String, slice: &[u8; 16]) -> Self {
        Self{
            word,
            path: slice.iter()
                .filter(|&n| n != &16)
                .cloned()
                .collect()
        }
    }
}

impl PartialOrd for FoundWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match self.word.len().cmp(&other.word.len()) {
                Ordering::Equal => self.word.cmp(&other.word),
                ord => ord,
            }
        )
    }
}