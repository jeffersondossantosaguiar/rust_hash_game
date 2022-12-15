use std::io;

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

        println!("Your move was: {player_move}");

        moves[0][0] = "X";

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
