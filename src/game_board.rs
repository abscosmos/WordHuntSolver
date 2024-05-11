use std::collections::HashSet;
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

    pub fn letters_distinct(&self) -> Vec<char> {
        self.0.iter()
            .flat_map(|row| row.iter().cloned())
            .unique()
            .collect()
    }

    pub fn find_possible_sequences(&self, word_set: &HashSet<String>) -> Vec<String> {
        let visited = [[false; 4]; 4];

        let mut found = vec![];

        for r in 0..4 {
            for c in 0..4 {
                self.find_possible_sequences_recurse(r, c, String::new(), visited, &mut found, word_set);
            }
        }

        found
    }

    fn find_possible_sequences_recurse(&self, row: i8, col: i8, mut word: String, mut visited: [[bool; 4]; 4], found: &mut Vec<String>, word_set: &HashSet<String>) {
        let valid_idx = 0..4i8;

        if !valid_idx.contains(&row) && !valid_idx.contains(&row) || visited[row as usize][col as usize] {
            return;
        }

        visited[row as usize][col as usize] = true;

        word.push(self.0[row as usize][col as usize]);
        // check if word list has word

        let move_set = [-1i8, 0, 1];
        let move_set = move_set
            .iter()
            .cartesian_product(move_set)
            .collect::<Vec<_>>();

        // println!("{}", word);
        if word_set.contains(&word) {
            found.push(word.clone());
        }

        for (&x, y) in move_set {
            let (c, r) = (x + col, y + row);

            if valid_idx.contains(&c) && valid_idx.contains(&r) && !visited[r as usize][c as usize] {
                self.find_possible_sequences_recurse(r, c, word.clone(), visited, found, word_set);
            }
        }
    }

    pub fn find_possible_sequences_trie(&self, word_trie: &Trie<u8>) -> Vec<String> {
        let visited = [[false; 4]; 4];

        let mut found = vec![];

        for r in 0..4 {
            for c in 0..4 {
                self.find_possible_sequences_recurse_trie(r, c, String::new(), visited, &mut found, word_trie);
            }
        }

        found
    }

    fn find_possible_sequences_recurse_trie(&self, row: i8, col: i8, mut word: String, mut visited: [[bool; 4]; 4], found: &mut Vec<String>, word_trie: &Trie<u8>) {
        let valid_idx = 0..4i8;

        if !valid_idx.contains(&row) && !valid_idx.contains(&row) || visited[row as usize][col as usize] {
            return;
        }

        visited[row as usize][col as usize] = true;

        word.push(self.0[row as usize][col as usize]);
        // check if word list has word

        let move_set = [-1i8, 0, 1];
        let move_set = move_set
            .iter()
            .cartesian_product(move_set)
            .collect::<Vec<_>>();

        if word_trie.is_prefix(&word) {
            for (&x, y) in move_set {
                let (c, r) = (x + col, y + row);
                if valid_idx.contains(&c) && valid_idx.contains(&r) && !visited[r as usize][c as usize] {
                    self.find_possible_sequences_recurse_trie(r, c, word.clone(), visited, found, word_trie);
                }
            }
        }

        if word_trie.exact_match(&word) {
            found.push(word.clone());
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

pub struct FoundWord(Vec<u8>);