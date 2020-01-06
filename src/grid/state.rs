use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq)]
pub enum State {
    X,
    O,
    N,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let c = match self {
            Self::X => 'X',
            Self::O => 'O',
            Self::N => ' ',
        };

        write!(f, "{}", c)
    }
}
