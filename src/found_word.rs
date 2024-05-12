use std::cmp::Ordering;
use std::fmt;
use itertools::Itertools;

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

impl fmt::Display for FoundWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = ['*'; 16];
        self.path.iter()
            .enumerate()
            .for_each(|(i, &n)| {
                board[n as usize] = ((i + 1) as u8 + b'0') as char;
            });

        let board = board.iter()
            .chunks(4)
            .into_iter()
            .map(|row| row.into_iter().join(" "))
            .join("\n");

        write!(f, "{}:\n{board}\n", self.word)
    }
}