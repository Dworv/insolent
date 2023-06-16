use std::io::{Read, Write, ErrorKind::UnexpectedEof};

use insolent::{LanguageError, LanguageErrorKind};

pub fn interpret<I: Iterator<Item = char>>(
    mut chars: I,
    input: Box<&mut dyn Read>,
    output: Box<&mut dyn Write>,
) -> Result<(), LanguageError> {
    todo!()
}