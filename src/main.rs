use crate::cli::read_cli_args;
use crate::configuration::{Configuration, Database, Difficulty, InputOutput};
use hangman::State;

mod cli;
mod configuration;
mod dictionary;
mod hangman;
mod io;

fn main() {
    // TODO Get configuration from command args
    let configuration = Configuration::new(
        Difficulty::Easy,
        InputOutput::StdIn,
        InputOutput::StdIn,
        Database::File,
    );
    let reader = io::create_reader(&configuration);
    let writer = io::create_writer(&configuration);
    // Create a game with the word to guess based on the difficulty.
    let mut game = read_cli_args(reader, writer, configuration);

    // Run one hangman game.
    game.start();

    // TODO Move this functionality to either the HangmanGame struct or the writer so that this is
    //  done at the end of the game automatically.
    // Display the results of the game.
    match game.get_state() {
        State::Win(_) => println!("You WIN!!!"),
        State::Lose => println!("You LOSE!!!"),
        _ => println!("Something went wrong"),
    }
}
