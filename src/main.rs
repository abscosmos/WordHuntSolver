use std::io;
use std::io::Write;
use itertools::Itertools;
use crate::game_board::GameBoard;
use crate::word_list::WordList;

mod game_board;
mod word_list;
mod found_word;

fn main() {
    let mut board= String::new();

    println!("Enter board as 16 continuous letters: ");
    io::stdin().read_line(&mut board).expect("Read input from stdin");

    let board = GameBoard::from_string(&board.trim()).expect("Invalid board input");

    println!("{board}\n");

    let mut list = WordList::load_from_file("dictionary/CollinsScrabbleWords.txt".as_ref()).unwrap();
    list.retain_only_buildable(&board.letters());
    let words_trie = board.find_possible_sequences(&list.into_trie());

    for w in words_trie.iter().sorted().rev() {
        println!("{w}\n");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
