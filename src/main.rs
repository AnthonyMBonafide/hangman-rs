use clap::{App, Arg};
use hangman::{HangmanGame, State};
use std::io::{self, BufRead};

mod dictionary;
mod display;
mod hangman;

fn main() {
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

    // Create a game with the word to guess based on the difficulty.
    let mut game = HangmanGame::new(dictionary::SimpleWordGenerator {});

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
