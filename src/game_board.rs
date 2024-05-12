use std::cmp::Ordering;
use std::fmt;
use itertools::Itertools;
use trie_rs::Trie;

#[derive(Debug)]
pub struct GameBoard([[char; 4]; 4]);
// TODO: make this a 1D slice eventually, with generic size params

impl GameBoard {
    pub fn from_string(letters: &str) -> Option<Self> {
        if letters.len() != 16 || !letters.chars().all(|c| c.is_ascii_alphabetic()) {
            return None;
        }

        let mut board = [[' '; 4]; 4];

        letters.chars()
            .map(|c| c.to_ascii_lowercase())
            .enumerate()
            .for_each(|(i, c)| board[i / 4][i % 4] = c);

        Some(GameBoard(board))
    }

    pub fn letters(&self) -> Vec<char> {
        self.0.iter()
            .flat_map(|row| row.iter().cloned())
            .collect()
    }

    pub fn find_possible_sequences(&self, word_trie: &Trie<u8>) -> Vec<FoundWord> {
        let mut found = Vec::new();

        for r in 0..4 {
            for c in 0..4 {
                self.find_possible_sequences_recurse(r, c, String::new(), [[false; 4]; 4], &mut found, word_trie, Vec::new());
            }
        }

        found
    }

    fn find_possible_sequences_recurse(&self, row: usize, col: usize, mut word: String, mut visited: [[bool; 4]; 4], found: &mut Vec<FoundWord>, word_trie: &Trie<u8>, mut path: Vec<u8>) {
        let valid_idx = 0..4;

        visited[row][col] = true;

        word.push(self.0[row][col]);
        path.push((row * 4 + col) as u8);

        let move_set = [-1i8, 0, 1];
        let move_set = move_set
            .iter()
            .cartesian_product(move_set);

        if word_trie.is_prefix(&word) {
            for (&y, x) in move_set {
                let (c, r) = ((x + col as i8) as usize, (y + row as i8) as usize);
                if valid_idx.contains(&c) && valid_idx.contains(&r) && !visited[r][c] {
                    self.find_possible_sequences_recurse(r, c, word.clone(), visited, found, word_trie, path.clone());
                }
            }
        }

        if word_trie.exact_match(&word) {
            found.push(FoundWord { word: word.clone(), path });
        }
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = self.0.iter()
            .map(|row| row.iter().join(" "))
            .join("\n");
        write!(f, "{board}")
    }
}

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