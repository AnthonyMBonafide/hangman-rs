use hangman::{HangmanGame, State};
use std::io::{self, BufRead};
use crate::cli::read_cli_args;

mod dictionary;
mod display;
mod hangman;
mod cli;

fn main() {
    // Create a game with the word to guess based on the difficulty.
    let mut game = read_cli_args();

    // Main game loop
    while match game.get_state() {
        State::Active(_) => true,
        _ => false,
    } {
        display::draw(game.get_state());
        println!("Secret Word: {}", game.display_word());
        println!("Incorrect guesses: {}", game.incorrect_guesses());
        println!("_______________________________");
        println!("Enter a your guess(letter): ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                break;
            }
        }

        let g = &buffer.chars().next().unwrap();
        game.guess(g);
    }

    display::draw(game.get_state());
    match game.get_state(){
        State::Win(_) => {println!("You WIN!!!")},
        State::Lose => {println!("You LOSE!!!")},
        _ => {println!("Something went wrong")},
    }
}
