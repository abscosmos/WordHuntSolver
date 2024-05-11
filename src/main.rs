use crate::game_board::GameBoard;

mod game_board;
mod dictionary;

fn load_word_list(path: &std::path::Path) -> std::io::Result<Vec<String>> {
    Ok(
        std::fs::read_to_string(path)?
            .lines()
            .map(ToString::to_string)
            .collect()
    )
}

fn get_possible_board_words(letters: &Vec<char>, words: &Vec<String>) -> Vec<String> {
    words.iter()
        .filter(|&word| word.chars().all(|c| letters.contains(&c)))
        .map(ToOwned::to_owned)
        .collect()
}

fn main() {
    let board = "oatrihpshtnrenei";

    let board = GameBoard::from_string(board).unwrap();

    println!("{board}");

    let words = load_word_list("dictionary/compiled_words.txt".as_ref()).unwrap();
    let allowed_words = get_possible_board_words(&board.letters(), &words);

    println!("{}", allowed_words.len());

    for word in allowed_words.iter().step_by(100).take(100) {
        println!("{word}");
    }
}
