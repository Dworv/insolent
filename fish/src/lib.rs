mod grid;
mod interpreter;

use std::io::{BufReader, Read, Write};

use utf8_chars::BufReadCharsExt;

pub(crate) use grid::Grid;

use insolent::{Language, LanguageError};
use interpreter::interpret;

pub struct Fish;

impl Language for Fish {
    fn name(&self) -> &'static str {
        return "fish";
    }

    fn interpret(
        &self,
        chars: &mut BufReader<&mut dyn Read>,
        input: Box<&mut dyn Read>,
        output: Box<&mut dyn Write>,
    ) -> Result<(), LanguageError> {
        interpret(
            chars,
            input,
            output
        )
    }
}
