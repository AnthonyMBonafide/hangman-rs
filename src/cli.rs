use crate::dictionary;
use crate::hangman::HangmanGame;
use crate::io::{ Reader, Writer};
use clap::{App, Arg};

pub fn read_cli_args<R, W>(reader : R, writer : W) -> HangmanGame<R,W> where R: Reader, W: Writer {
    // Parse CLI command arguments
    let matches = App::new("Hangman")
        .version("1.0")
        .author("Anthony M. Bonafide <AnthonyMBonafide@gmail.com>")
        .about("A CLI version of hangman.")
        .arg(
            Arg::with_name("difficulty")
                .short("d")
                .long("difficulty")
                .value_name("difficulty")
                .help("Sets the difficulty of the game")
                .possible_values(&["easy".as_ref(), "medium".as_ref(), "hard".as_ref()])
                .default_value("easy")
                .takes_value(true),
        )
        .get_matches();

    // Get the difficulty from the parse CLI arguments.
    let difficulty = matches.value_of("difficulty").unwrap_or("easy");
    println!("Value for difficulty: {}", difficulty);

    HangmanGame::new(dictionary::SimpleWordGenerator {}, reader, writer)
}
