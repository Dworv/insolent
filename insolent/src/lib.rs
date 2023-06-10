use std::io::{BufReader, Read, Write};

pub trait Language {
    fn name(&self) -> &'static str;
    fn interpret(
        &self,
        input: &mut BufReader<&mut dyn Read>,
        input: Box<&mut dyn Read>,
        output: Box<&mut dyn Write>,
    ) -> Result<(), LanguageError>;
}

#[derive(Debug)]
pub struct LanguageError {
    pub kind: LanguageErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum LanguageErrorKind {
    Syntax,
    Runtime,
}
