use std::io::{ErrorKind::UnexpectedEof, Read, Write, BufReader};

use insolent::{LanguageError, LanguageErrorKind};

pub fn interpret(
    chars: &mut BufReader<&mut dyn Read>,
    input: Box<&mut dyn Read>,
    output: Box<&mut dyn Write>,
) -> Result<(), LanguageError> {
    todo!()
}
