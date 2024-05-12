use std::fmt;
use itertools::Itertools;
use crate::found_word::FoundWord;
use crate::word_list::WordList;

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

    pub fn find_possible_sequences_with_duplicates(&self, word_list: &WordList) -> Vec<FoundWord> {
        let mut found = Vec::new();

        for r in 0..4 {
            for c in 0..4 {
                self.find_possible_sequences_recurse(r, c, String::new(), [[false; 4]; 4], &mut found, word_list, Vec::new());
            }
        }

        found
    }

    pub fn find_possible_sequences(&self, word_list: &WordList) -> Vec<FoundWord> {
        self.find_possible_sequences_with_duplicates(word_list)
            .into_iter()
            .unique_by(|a| a.word.clone())
            .collect()
    }

    fn find_possible_sequences_recurse(&self, row: usize, col: usize, mut word: String, mut visited: [[bool; 4]; 4], found: &mut Vec<FoundWord>, word_list: &WordList, mut path: Vec<u8>) {
        let valid_idx = 0..4;

        visited[row][col] = true;

        word.push(self.0[row][col]);
        path.push((row * 4 + col) as u8);

        let move_set = [-1i8, 0, 1];
        let move_set = move_set
            .iter()
            .cartesian_product(move_set);

        if word_list.0.is_prefix(&word) {
            for (&y, x) in move_set {
                let (c, r) = ((x + col as i8) as usize, (y + row as i8) as usize);
                if valid_idx.contains(&c) && valid_idx.contains(&r) && !visited[r][c] {
                    self.find_possible_sequences_recurse(r, c, word.clone(), visited, found, word_list, path.clone());
                }
            }
        }

        if word_list.0.exact_match(&word) {
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