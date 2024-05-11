use crate::game_board::GameBoard;

mod game_board;

fn main() {
    let board = "oatrihpshtnrenei";

    let board = GameBoard::from_string(board).unwrap();

    println!("{board}");
}
