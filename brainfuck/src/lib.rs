mod interpreter;

use std::io::{BufReader, Read, Write};

use utf8_chars::BufReadCharsExt;

use insolent::{Language, LanguageError};
use interpreter::interpret;

pub struct Brainfuck;

impl Language for Brainfuck {
    fn name(&self) -> &'static str {
        return "brainfuck";
    }

    fn interpret(
        &self,
        chars: &mut BufReader<&mut dyn Read>,
        input: Box<&mut dyn Read>,
        output: Box<&mut dyn Write>,
    ) -> Result<(), LanguageError> {
        interpret(chars.chars().map(|x| x.unwrap()), input, output)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, empty, sink};

    use insolent::Language;

    use crate::Brainfuck;
    
    #[test]
    fn hello_word_runs() {
        let mut code = &include_bytes!("test_code/hello_world.b")[..];
        Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut sink())).unwrap();
    }

    #[test]
    fn hello_word_runs_correctly() {
        let mut code = &include_bytes!("test_code/hello_world.b")[..];
        let mut output = String::from("                    ");
        Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
        assert_eq!(
            output.trim().to_string(),
            "Hello World!"
        )
    }

    #[test]
    fn character_comments_ignored() {
        let mut code = &include_bytes!("test_code/commented_code.b")[..];
        let mut output = String::from("                    ");
        Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
        assert_eq!(
            output.trim().to_string(),
            "!"
        )
    }

    #[test]
    fn first_loop_comments_ignored() {
        let mut code = &include_bytes!("test_code/commented_code_2.b")[..];
        let mut output = String::from("                    ");
        Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
        assert_eq!(
            output.trim().to_string(),
            "#"
        )
    }
}
