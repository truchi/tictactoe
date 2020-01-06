use std::fmt::{Display, Formatter, Result};

pub mod state;
use self::state::State;

pub struct Grid {
    grid: [[State; 3]; 3],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: [
                [State::N, State::N, State::N],
                [State::N, State::N, State::N],
                [State::N, State::N, State::N],
            ],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &State {
        &self.grid[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, state: &State) {
        self.grid[x][y] = match state {
            State::X => State::X,
            State::O => State::O,
            State::N => State::N,
        };
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn winner(&self) -> Option<State> {
        let winner = |arr: [&State; 3]| {
            let win = arr[0] == arr[1] && arr[1] == arr[2];

            if win {
                match arr[0] {
                    State::X => Some(State::X),
                    State::O => Some(State::O),
                    State::N => None,
                }
            } else {
                None
            }
        };

        for line in 0..3 {
            let win = winner(self.line(line));
            if win.is_some() {
                return win;
            }
        }

        for row in 0..3 {
            let win = winner(self.row(row));
            if win.is_some() {
                return win;
            }
        }

        let win = winner(self.diag1());
        if win.is_some() {
            return win;
        }

        let win = winner(self.diag2());
        if win.is_some() {
            return win;
        }

        None
    }

    fn line(&self, x: usize) -> [&State; 3] {
        [&self.get(x, 0), &self.get(x, 1), &self.get(x, 2)]
    }

    fn row(&self, y: usize) -> [&State; 3] {
        [&self.get(0, y), &self.get(1, y), &self.get(2, y)]
    }

    fn diag1(&self) -> [&State; 3] {
        [&self.get(0, 0), &self.get(1, 1), &self.get(2, 2)]
    }

    fn diag2(&self) -> [&State; 3] {
        [&self.get(0, 2), &self.get(1, 1), &self.get(2, 0)]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let [[p11, p12, p13], [p21, p22, p23], [p31, p32, p33]] = &self.grid;

        write!(
            f,
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            p11, p12, p13, p21, p22, p23, p31, p32, p33
        )
    }
}
