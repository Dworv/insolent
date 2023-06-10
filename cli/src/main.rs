use std::io::{stdin, stdout, BufReader};

use brainfuck::Brainfuck;
use insolent::Language;

fn main() {
    let code = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.".to_string();
    Brainfuck::interpret(&Brainfuck, &mut BufReader::new(&mut code.as_bytes()), Box::new(&mut stdin()), Box::new(&mut stdout())).unwrap();
}
