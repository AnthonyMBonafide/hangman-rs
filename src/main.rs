use clap::{App, Arg};
use hangman::{HangmanGame, State};
use std::io::{self, BufRead};

mod hangman;
mod display;

fn main() {
    // Parse CLI command arguments
    let matches = App::new("Hangman")
        .version("1.0")
        .author("Anthony M. Bonafide <AnthonyMBonafide@gmail.com>")
        .about("A CLI version of hangman.")
        .arg(Arg::with_name("difficulty")
            .short("d")
            .long("difficulty")
            .value_name("difficulty")
            .help("Sets the difficulty of the game")
            .possible_values(&["easy".as_ref(), "medium".as_ref(), "hard".as_ref()])
            .default_value("easy")
            .takes_value(true))
        .get_matches();

    // Get the difficulty from the parse CLI arguments.
    let difficulty = matches.value_of("difficulty").unwrap_or("easy");
    println!("Value for difficulty: {}", difficulty);

    //TODO Get word based on difficulty.

    // Create a game with the word to guess based on the difficulty.
    let mut game = HangmanGame::new(String::from("test"), 7);

    // Main game loop
    //TODO move game loop inside HangmanGame. We should only be asking the user if they want to
    // play again in this loop.
    while match game.get_state() {
        State::Active(_) => { true }
        _ => false,
    } {
        // TODO Move all functionality within the HangMan game
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
    println!("Secret Word: {}", game.display_word());
    println!("YOU WIN!!!")
}
