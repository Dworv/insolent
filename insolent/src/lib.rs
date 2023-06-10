use std::io::{Read, Write};

pub trait Language {
    fn name(&self) -> &'static str;
    fn interpret(&self, input: dyn Read, flags: Vec<String>) -> Result<Box<dyn Write>, LanguageError>;
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
    Runtime
}