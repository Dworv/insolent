use std::io::{BufReader, BufRead, Read};

pub struct Grid {
    lines: Vec<String>
}

impl Grid {
    pub fn new(chars: &mut BufReader<&mut dyn Read>) -> Self {
        Self {
            lines: chars.lines().map(|x| x.unwrap()).collect()
        }
    }

    pub fn get_char(&self, r: usize, c: usize) -> Option<char> {
        self
            .lines
            .get(r)?
            .chars()
            .nth(c)
    }
}