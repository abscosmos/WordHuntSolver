use std::fmt;
use itertools::Itertools;

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