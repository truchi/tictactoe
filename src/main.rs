mod grid;

use grid::state::State;
use grid::Grid;
use std::io;
use std::process::Command;

fn main() {
    let mut player = State::X;
    let mut played = vec![];
    let mut grid = Grid::new();

    let clear = || {
        if Command::new("clear").status().unwrap().success() {
            println!("screen successfully cleared");
        }
    };

    let ask = || {
        let mut play = String::new();

        io::stdin()
            .read_line(&mut play)
            .expect("Failed to read line");

        play.trim().to_owned()
    };

    let coords = |play: &String| match play.as_str() {
        "11" => Some((0, 0)),
        "12" => Some((0, 1)),
        "13" => Some((0, 2)),
        "21" => Some((1, 0)),
        "22" => Some((1, 1)),
        "23" => Some((1, 2)),
        "31" => Some((2, 0)),
        "32" => Some((2, 1)),
        "33" => Some((2, 2)),
        _ => None,
    };

    loop {
        clear();

        if let Some(winner) = grid.winner() {
            println!("Winner: {}", winner);
            grid.print();
            break;
        } else {
            println!("{} to play", player);
            grid.print();
        }

        let play = ask();

        if played.contains(&play) {
            continue;
        }

        let (x, y) = match coords(&play) {
            Some(play) => play,
            None => continue,
        };

        played.push(play);

        grid.set(x, y, &player);

        player = match player {
            State::X => State::O,
            State::O => State::X,
            State::N => State::N,
        }
    }
}
