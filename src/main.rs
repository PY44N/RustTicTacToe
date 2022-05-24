mod board;
mod util;

fn main() {
    let mut current_board = board::Board::new();

    let mut moves_taken = 0;
    loop {
        current_board.print();

        if current_board.is_board_full() {
            println!("Its a tie!");
            break;
        }

        let winner = current_board.get_winner();
        if winner != 0 {
            println!(
                "Congratulations {} you won!",
                if winner == 1 { "X" } else { "O" }
            );
            break;
        }

        let input = util::input::get_position(&current_board, moves_taken);

        current_board.set_value(&input, (moves_taken % 2 + 1) as u8);

        moves_taken += 1;
    }
}
