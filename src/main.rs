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

    let sequences = board.find_possible_sequences();
    let fin = std::time::Instant::now() - start;


    let start2 = std::time::Instant::now();

    let real_words: Vec<String> = sequences.clone()
        .into_iter()
        .filter(|w| list.words().contains(w))
        .sorted_by(|a, b| b.len().cmp(&a.len()))
        .collect();
    let fin2 = std::time::Instant::now() - start2;

    for word in &real_words {
        println!("{word}");
    }

    println!("{} total, {} sec, {}real", sequences.len(), fin.as_secs(), real_words.len());
    println!("{} sec filter", fin2.as_secs());
}
