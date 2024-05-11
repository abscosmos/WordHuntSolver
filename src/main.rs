use crate::game_board::GameBoard;
use crate::word_list::WordList;

mod game_board;
mod dictionary;
mod word_list;

fn main() {
    let board = "oatrihpshtnrenei";

    let board = GameBoard::from_string(board).unwrap();

    println!("{board}");

    let mut words = WordList::load_from_file("dictionary/compiled_words.txt".as_ref()).unwrap();
    words.retain_only_possible(&board.letters_distinct());

    println!("{}", words.words().len());

    for word in words.words().iter().step_by(100).take(100) {
        println!("{word}");
    }
}
