use crate::dictionary;
use crate::hangman::HangmanGame;
use crate::io::CommandLineIO;
use clap::{App, Arg};

// TODO Fix the generics so that this function is flexible enough to create any type of Hangman game.
//  Maybe move the creation of the reader and writer outside of this function and then we can have
//  generic input and output. A separate function can handle creating the reader and writer and the
//  main function can treat it as a generic and pass it to this function, but the compiler would
//  know the type since the creation function will explicitly have something like a match statement.
pub fn read_cli_args() -> HangmanGame<CommandLineIO, CommandLineIO> {
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

    // Reader/Writer
    let reader = CommandLineIO {};
    let writer = CommandLineIO {};

    HangmanGame::new(dictionary::SimpleWordGenerator {}, reader, writer)
}
