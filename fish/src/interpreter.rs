use std::io::{ErrorKind::UnexpectedEof, Read, Write, BufReader};

use insolent::{LanguageError, LanguageErrorKind};

use crate::grid::Grid;

pub fn interpret(
    chars: &mut BufReader<&mut dyn Read>,
    input: Box<&mut dyn Read>,
    output: Box<&mut dyn Write>,
) -> Result<(), LanguageError> {
    let grid = Grid::new(chars);
    let mut state = State::default();

    loop {
        match state.mode {
            Mode::Normal => {
                
            },
            Mode::Characters => todo!(),
        }
    }
}

#[derive(Default)]
struct State {
    row: usize,
    col: usize,
    dir: Direction,
    mode: Mode
}

#[derive(Default)]
enum Mode {
    #[default]
    Normal,
    Characters
}

#[derive(Default)]
enum Direction {
    #[default]
    Right,
    Up,
    Down,
    Left,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (-1, 0)
        }
    }
}