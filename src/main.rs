use std::io;
use itertools::Itertools;
use crate::game_board::GameBoard;
use crate::word_list::WordListBuilder;

mod game_board;
mod word_list;
mod found_word;

fn main() {
    let word_list = WordListBuilder::from_words(
        include_str!("../dictionary/CollinsScrabbleWords.txt")
            .lines()
    );

    'main: loop {
        let mut board= String::new();
        println!("Enter board as 16 continuous letters: ");
        io::stdin().read_line(&mut board).expect("Read input from stdin");

        let Some(board) = GameBoard::from_string(&board.trim()) else {
            continue;
        };

        let found = board.find_possible_sequences(
            &word_list
                .clone()
                .only_using_letters(&board.letters())
                .build()
        );

        for f in found.iter().sorted().rev() {
            println!("{f}");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "new" => break,
                "stop" => break 'main,
                _ => {},
            }
        }
    }
}
