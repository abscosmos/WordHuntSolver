use itertools::Itertools;
use crate::game_board::GameBoard;
use crate::word_list::WordList;

mod game_board;
mod dictionary;
mod word_list;

fn main() {
    let board = "oatrihpshtnrenei";

    let board = GameBoard::from_string(board).unwrap();

    println!("{board}");

    let mut list = WordList::load_from_file("dictionary/compiled_words.txt".as_ref()).unwrap();
    list.retain_only_possible(&board.letters_distinct());

    println!("{}", list.words().len());

    let start = std::time::Instant::now();

    let f = board.find_possible_sequences();
    let fin = std::time::Instant::now() - start;

    // for w in list.words().iter() {
    //     if f.contains(w) {
    //         println!("{}", w);
    //         ct += 1;
    //     } else {
    //         nct += 1;
    //         if (nct % 100) == 0 {
    //             println!("{}", nct);
    //         }
    //     }
    // }

    println!("{} total, {} sec", f.len(), fin.as_secs());
}
