use crate::cli::read_cli_args;
use hangman::State;

mod cli;
mod dictionary;
mod hangman;
mod io;

fn main() {
    let reader = io::create_reader();
    let writer = io::create_writer();
    // Create a game with the word to guess based on the difficulty.
    let mut game = read_cli_args(reader, writer);

    // Run one hangman game.
    game.start();

    // Display the results of the game.
    match game.get_state() {
        State::Win(_) => println!("You WIN!!!"),
        State::Lose => println!("You LOSE!!!"),
        _ => println!("Something went wrong"),
    }
}
