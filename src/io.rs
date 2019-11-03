use crate::display;
use crate::hangman::HangmanGame;
use std::io::{self, BufRead};

pub trait Reader {
    fn read_guess(&self) -> Result<char, String>;
}

pub trait Writer {
    fn write_game_state<R, W>(&self, game: &HangmanGame<R, W>)
    where
        R: Reader,
        W: Writer;
}

pub struct CommandLineIO {}

impl Reader for CommandLineIO {
    fn read_guess(&self) -> Result<char, String> {
        println!("Enter a your guess(letter): ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer);
        let g = &buffer.chars().next().unwrap();
        Result::Ok(g.to_owned())
    }
}

impl Writer for CommandLineIO {
    fn write_game_state<R, W>(&self, game: &HangmanGame<R, W>)
    where
        R: Reader,
        W: Writer,
    {
        display::draw(game.get_state());
        println!("Secret Word: {}", game.display_word());
        println!("Incorrect guesses: {}", game.incorrect_guesses());
        println!("_______________________________");
    }
}
