use std::io;

fn main() {
    let mut moves = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]];
    let mut player: i8 = 1;

    loop {
        board(&moves);

        println!(
            "Player's {} turn. Tip: Choose Column and Line Ex: 0A",
            player
        );

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

        match moves[line][column] {
            "-" => {
                moves[line][column] = match player {
                    1 => "O",
                    _ => "X",
                };
            }
            _ => {
                println!("*** This field already has a move! ***");
                continue;
            }
        }

        match has_winner(&moves, &moves[line][column]) {
            true => {
                println!("*** Congratulations!!!, Player {} winner ***", { player });
                board(&moves);
                break;
            }
            _ => {
                player = match player {
                    1 => 2,
                    _ => 1,
                }
            }
        }
    }
}

fn board(moves: &[[&str; 3]]) {
    let title = "HASH GAME";
    let bar = "|";

    println!("{: >2}{:^13}", "", title);
    println!("{: >5}{: >4}{: >4}", "0", "1", "2");
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

fn get_column_value(player_move: &String) -> Result<usize, ()> {
    match player_move
        .chars()
        .nth(0)
        .expect("Invalid character")
        .to_digit(10)
    {
        Some(column) if (0..=2).contains(&column) => Ok(column as usize),
        _ => Err(()),
    }
}

fn get_line_value(player_move: &String) -> Result<usize, ()> {
    match player_move.chars().nth(1).expect("Invalid character") {
        'A' | 'a' => Ok(0),
        'B' | 'b' => Ok(1),
        'C' | 'c' => Ok(2),
        _ => Err(()),
    }
}

fn has_winner(moves: &[[&str; 3]], char: &&str) -> bool {
    if moves[0].iter().all(|x| x == char) {
        return true;
    }
    if moves[1].iter().all(|x| x == char) {
        return true;
    }
    if moves[2].iter().all(|x| x == char) {
        return true;
    }
    if [moves[0][0], moves[1][0], moves[2][0]]
        .iter()
        .all(|x| x == char)
    {
        return true;
    }
    if [moves[0][1], moves[1][1], moves[2][1]]
        .iter()
        .all(|x| x == char)
    {
        return true;
    }
    if [moves[0][2], moves[1][2], moves[2][2]]
        .iter()
        .all(|x| x == char)
    {
        return true;
    }
    if [moves[0][0], moves[1][1], moves[2][2]]
        .iter()
        .all(|x| x == char)
    {
        return true;
    }
    if [moves[0][2], moves[1][1], moves[2][0]]
        .iter()
        .all(|x| x == char)
    {
        return true;
    }
    return false;
}
