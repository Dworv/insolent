use std::io::{BufReader, empty};

use brainfuck::Brainfuck;
use insolent::Language;

#[test]
fn print_grad() {
    let mut code = &include_bytes!("grad.b")[..];
    let mut output = String::from("                    ");
    dbg!("interpretting");
    Brainfuck.interpret(&mut BufReader::new(&mut code), Box::new(&mut empty()), Box::new(&mut unsafe { output.as_bytes_mut() })).unwrap();
    assert_eq!(
        output.trim_end().to_string(),
        "grad"
    )
}