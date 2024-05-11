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

    let mut words = board.find_possible_sequences(&list.words());
    let words_trie = board.find_possible_sequences_trie(&list.clone().into_trie());

    println!("original: {}, trie: {}", words.len(), words_trie.len());

    let same = words.iter()
        .sorted().eq(words_trie.iter().sorted());

    println!("Are both lists the same? {same}");
}
