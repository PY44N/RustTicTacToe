use std::io;

use crate::board::Board;

use super::vector2;

pub fn get_position(current_board: &Board, moves_taken: u16) -> vector2::Vector2 {
    loop {
        println!(
            "Player {} is up",
            if moves_taken % 2 == 0 { "X" } else { "O" }
        );

        let mut x = String::new();
        let mut y = String::new();

        println!("What x position would you like?");

        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read x value");

        let x: usize = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to read x value");
                continue;
            }
        };

        if x >= 3 {
            println!("X value too large");
            continue;
        }

        println!("What y position would you like?");

        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read y value");

        let y: usize = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to read y value");
                continue;
            }
        };

        if y >= 3 {
            println!("Y value too large");
            continue;
        }

        if current_board.get_value(x, y) != 0 {
            println!("Position already taken!");
            continue;
        }

        return vector2::Vector2 { x, y };
    }
}
