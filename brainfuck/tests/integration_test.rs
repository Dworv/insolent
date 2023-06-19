use std::io::{empty, sink, BufReader};

use brainfuck::Brainfuck;
use insolent::{Language, LanguageError, LanguageErrorKind};

#[test]
fn print_grad() {
    let mut code = &include_bytes!("grad.b")[..];
    let mut output = String::from("                    ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim_end().to_string(), "grad")
}

#[test]
fn run_staircase() {
    let mut code = &include_bytes!("staircase.b")[..];
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut sink()),
        )
        .unwrap();
}

#[test]
fn all_symbols_run() {
    let mut code = &include_bytes!("all_symbols.b")[..];
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut sink()),
        )
        .unwrap();
}

#[test]
fn encoding_works() {
    let mut code = &include_bytes!("encodings.b")[..];
    let mut output = String::from("                    ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim_end().to_string(), " !\"#$%&'()")
}

#[test]
fn hello_word_runs() {
    let mut code = &include_bytes!("hello_world.b")[..];
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut sink()),
        )
        .unwrap();
}

#[test]
fn hello_word_runs_correctly() {
    let mut code = &include_bytes!("hello_world.b")[..];
    let mut output = String::from("                    ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "Hello World!")
}

#[test]
fn character_comments_ignored() {
    let mut code = &include_bytes!("commented_code.b")[..];
    let mut output = String::from("                    ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "!")
}

#[test]
fn first_loop_comments_ignored() {
    let mut code = &include_bytes!("commented_code_2.b")[..];
    let mut output = String::from("                    ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut empty()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "#")
}

#[test]
fn inputs_work() {
    let mut code = &include_bytes!("input.b")[..];
    let input = String::from("a");
    let mut output = String::from(" ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut input.as_bytes()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "a")
}

#[test]
fn check_even_digit_4() {
    let mut code = &include_bytes!("check_even_digit.b")[..];
    let input = String::from("4");
    let mut output = String::from(" ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut input.as_bytes()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "4")
}

#[test]
fn check_even_digit_8() {
    let mut code = &include_bytes!("check_even_digit.b")[..];
    let input = String::from("8");
    let mut output = String::from(" ");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut input.as_bytes()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "8")
}

#[test]
fn check_even_digit_odd() {
    let mut code = &include_bytes!("check_even_digit.b")[..];
    let input = String::from("3");
    let mut output = String::from("");
    Brainfuck
        .interpret(
            &mut BufReader::new(&mut code),
            Box::new(&mut input.as_bytes()),
            Box::new(&mut unsafe { output.as_bytes_mut() }),
        )
        .unwrap();
    assert_eq!(output.trim().to_string(), "")
}

#[test]
fn error_reports_correctly() {
    let mut code = &include_bytes!("hello_world.b")[..];
    assert_eq!(
        Brainfuck
            .interpret(
                &mut BufReader::new(&mut code),
                Box::new(&mut empty()),
                Box::new(&mut sink()),
            )
            .unwrap_err(),
        LanguageError {
            kind: LanguageErrorKind::Syntax,
            message: "Unmatched loop".to_string(),
            line: 5,
            column: 5
        }
    )
}