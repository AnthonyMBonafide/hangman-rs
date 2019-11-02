use crate::cli::read_cli_args;
use crate::io::{CommandLineIO, Reader, Writer};
use hangman::State;

mod cli;
mod dictionary;
mod display;
mod hangman;
mod io;

fn main() {
    // Create a game with the word to guess based on the difficulty.
    let mut game = read_cli_args();

    // Reader/Writer
    let reader = CommandLineIO {};
    let writer = CommandLineIO {};

    // Main game loop
    while match game.get_state() {
        State::Active(_) => true,
        _ => false,
    } {
        writer.write_game_state(&game);
        let guess_result = reader.read_guess();
        match guess_result {
            Ok(guess) => game.guess(&guess),
            Err(message) => {
                println!("Error: {}", message);
                break;
            }
        }
    }

    writer.write_game_state(&game);
    match game.get_state() {
        State::Win(_) => println!("You WIN!!!"),
        State::Lose => println!("You LOSE!!!"),
        _ => println!("Something went wrong"),
    }
}
