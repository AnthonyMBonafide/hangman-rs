use crate::configuration::{Configuration, InputOutput};
use crate::hangman::State;
use crate::hangman::State::{Active, Lose, Win};
use std::io::{self, BufRead};

pub trait Reader {
    fn read_guess(&self) -> Result<char, String>;
}

pub trait Writer {
    fn write_game_state(&self, game_state: State, display_word: String, incorrect_guesses: String);
}

pub fn create_reader(configuration: &Configuration) -> impl Reader {
    match configuration.get_input() {
        InputOutput::StdIn => CommandLineIO {},
        _ => unimplemented!(),
    }
}

pub fn create_writer(configuration: &Configuration) -> impl Writer {
    match configuration.get_input() {
        InputOutput::StdIn => CommandLineIO {},
        _ => unimplemented!(),
    }
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

impl CommandLineIO {
    // TODO Cleanup this redundant code.
    fn draw(game_state: State) {
        match game_state {
            Win(x) => match x {
                0 => println!("{}", BASE),
                1 => println!("{}", HEAD),
                2 => println!("{}", NECK),
                3 => println!("{}", LEFT_ARM),
                4 => println!("{}", RIGHT_ARM),
                5 => println!("{}", TORSO),
                6 => println!("{}", LEFT_LEG),
                _ => println!("{}", GAME_OVER),
            },
            Lose => println!("{}", GAME_OVER),
            Active(n) => match n {
                0 => println!("{}", BASE),
                1 => println!("{}", HEAD),
                2 => println!("{}", NECK),
                3 => println!("{}", LEFT_ARM),
                4 => println!("{}", RIGHT_ARM),
                5 => println!("{}", TORSO),
                6 => println!("{}", LEFT_LEG),
                _ => println!("{}", GAME_OVER),
            },
        }
    }
}

impl Writer for CommandLineIO {
    fn write_game_state(&self, game_state: State, display_word: String, incorrect_guesses: String) {
        CommandLineIO::draw(game_state);
        println!("Secret Word: {}", display_word);
        println!("Incorrect guesses: {}", incorrect_guesses);
        println!("_______________________________");
    }
}

const GAME_OVER: &str = "
     _______________
    |               |
   ( )              |
   \\|/              |
    |               |
   / \\              |
                    |
                    |
               _____|______

    ";

const BASE: &str = "
     _______________
    |               |
                    |
                    |
                    |
                    |
                    |
                    |
               _____|______

    ";

const HEAD: &str = "
     _______________
    |               |
   ( )              |
                    |
                    |
                    |
                    |
                    |
               _____|______

    ";

const NECK: &str = "
     _______________
    |               |
   ( )              |
    |               |
                    |
                    |
                    |
                    |
               _____|______

    ";
const LEFT_ARM: &str = "
     _______________
    |               |
   ( )              |
   \\|               |
                    |
                    |
                    |
                    |
               _____|______

    ";

const RIGHT_ARM: &str = "
     _______________
    |               |
   ( )              |
   \\|/              |
                    |
                    |
                    |
                    |
               _____|______

    ";

const TORSO: &str = "
     _______________
    |               |
   ( )              |
   \\|/              |
    |               |
                    |
                    |
                    |
               _____|______

    ";

const LEFT_LEG: &str = "
     _______________
    |               |
   ( )              |
   \\|/              |
    |               |
   /                |
                    |
                    |
               _____|______

    ";
