use itertools::Itertools;
use crate::game_board::GameBoard;
use crate::word_list::WordList;

mod game_board;
mod word_list;

fn main() {
    let board = "oatrihpshtnrenei";

    let board = GameBoard::from_string(board).unwrap();

    println!("{board}");

    let mut list = WordList::load_from_file("dictionary/compiled_words.txt".as_ref()).unwrap();
    list.retain_only_possible(&board.letters_distinct());

    println!("Potential words: {}", list.words().len());

    let start = std::time::Instant::now();
    let words = board.find_possible_sequences(list.words());
    let fin = std::time::Instant::now() - start;

    words.iter()
        .for_each(|w| println!("{w}"));

    println!("{} found, {} sec", words.len(), fin.as_secs());
}
