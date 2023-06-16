use std::io::{BufReader, empty, sink};

use brainfuck::Brainfuck;
use insolent::Language;

#[test]
fn print_grad() {
    let mut code = &include_bytes!("grad.b")[..];
    let mut output = String::from("                    ");
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
    assert_eq!(
        output.trim_end().to_string(),
        "grad"
    )
}

#[test]
fn run_staircase() {
    let mut code = &include_bytes!("staircase.b")[..];
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut sink())).unwrap();
}


#[test]
fn all_symbols_run() {
    let mut code = &include_bytes!("all_symbols.b")[..];
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut sink())).unwrap();
}

#[test]
fn encoding_works() {
    let mut code = &include_bytes!("encodings.b")[..];
    let mut output = String::from("                    ");
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
    assert_eq!(
        output.trim_end().to_string(),
        " !\"#$%&'()"
    )
}

#[test]
fn hello_word_runs() {
    let mut code = &include_bytes!("hello_world.b")[..];
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut sink())).unwrap();
}

#[test]
fn hello_word_runs_correctly() {
    let mut code = &include_bytes!("hello_world.b")[..];
    let mut output = String::from("                    ");
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
    assert_eq!(
        output.trim().to_string(),
        "Hello World!"
    )
}

#[test]
fn character_comments_ignored() {
    let mut code = &include_bytes!("commented_code.b")[..];
    let mut output = String::from("                    ");
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
    assert_eq!(
        output.trim().to_string(),
        "!"
    )
}

#[test]
fn first_loop_comments_ignored() {
    let mut code = &include_bytes!("commented_code_2.b")[..];
    let mut output = String::from("                    ");
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
    assert_eq!(
        output.trim().to_string(),
        "#"
    )
}
