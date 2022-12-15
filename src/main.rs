use std::{error::Error, io};

fn main() {
    let mut moves = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]];
    let mut player: i8 = 1;

    loop {
        board(&moves);

        println!("Please make your move Player {}. Ex: 1A", player);

        let mut player_move = String::new();

        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        let column = match get_column_value(&player_move) {
            Ok(value) => value,
            _ => {
                println!("Invalid column!");
                continue;
            }
        };

        let line = match get_line_value(&player_move) {
            Ok(value) => value,
            _ => {
                println!("Invalid line!");
                continue;
            }
        };

        println!("Column {}", column);
        println!("Line {}", line);

        player = match player {
            1 => 2,
            _ => 1,
        }
    }
}

fn board(moves: &[[&str; 3]]) {
    let title = "HASH GAME";
    let bar = "|";

    println!("{: >2}{:^13}", "", title);
    println!("{: >5}{: >4}{: >4}", "1", "2", "3");
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
    println!(
        "A {: <2}{}{: >2}{: >2}{: >2}{: >2}{: >2}",
        bar, moves[0][0], bar, moves[0][1], bar, moves[0][2], bar
    );
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
    println!(
        "B {: <2}{}{: >2}{: >2}{: >2}{: >2}{: >2}",
        bar, moves[1][0], bar, moves[1][1], bar, moves[1][2], bar
    );
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
    println!(
        "C {: <2}{}{: >2}{: >2}{: >2}{: >2}{: >2}",
        bar, moves[2][0], bar, moves[2][1], bar, moves[2][2], bar
    );
    println!("{: >2}+{:-^3}+{:-^3}+{:-^3}+", "", "", "", "");
}

fn get_column_value(player_move: &String) -> Result<i8, ()> {
    match player_move
        .chars()
        .nth(0)
        .expect("Invalid character")
        .to_digit(10)
    {
        Some(column) if (1..=3).contains(&column) => Ok(column as i8),
        _ => Err(()),
    }
}

fn get_line_value(player_move: &String) -> Result<i8, ()> {
    match player_move.chars().nth(1).expect("Invalid character") {
        'A' | 'a' => Ok(1),
        'B' | 'b' => Ok(2),
        'C' | 'c' => Ok(3),
        _ => Err(()),
    }
}
