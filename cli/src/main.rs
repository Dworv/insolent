use std::{process::exit, io::{stdin, stdout, BufReader}, path::PathBuf, fs::File};

use brainfuck::Brainfuck;
use clap::{Parser, Command, FromArgMatches as _, Args};
use insolent::Language;

/// A tool to interpret code written in esotoric programming languages
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg()]
    filename: PathBuf,

    #[arg(short, long)]
    lang: String
}

fn main() {
    let languages: Vec<&dyn Language> = vec![
        &Brainfuck
    ];

    let names: Vec<&'static str> = vec![
        "brainfuck"
    ];

    let cli = get_cli(names);

    let mut lang = None;
    for l in languages {
        if l.name() == cli.lang {
            lang = Some(l);
        }
    }

    if let Some(lang) = lang {
        lang.interpret(&mut BufReader::new(&mut File::open(cli.filename).unwrap()), Box::new(&mut stdin()), Box::new(&mut stdout())).unwrap();
    }
}

fn get_cli(allowed_langs: Vec<&'static str>) -> Cli {
    let cli = Cli::augment_args(Command::new("")).mut_arg("lang", |arg| arg.value_parser(allowed_langs)); 
    Cli::from_arg_matches(&cli.get_matches())
        .map_err(|err| err.exit())
        .unwrap()
}

fn deduce_lang(filename: &str) {
    
}