use itertools::Itertools;
use crate::game_board::{FoundWord, GameBoard};
use crate::word_list::WordList;

mod game_board;
mod word_list;

fn main() {
    let board = "oatrihpshtnrenei";

    let board = GameBoard::from_string(board).unwrap();

    println!("{board}");

    let mut list = WordList::load_from_file("dictionary/compiled_words.txt".as_ref()).unwrap();
    list.retain_only_buildable(&board.letters());
    let words_trie = board.find_possible_sequences(&list.into_trie());

    words_trie.iter()
        .sorted()
        .rev()
        .take(5)
        .for_each(|w| println!("{w:?}"));

    println!("{}", words_trie.len());
}
