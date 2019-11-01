use crate::hangman::State;
use crate::hangman::State::{Active,Lose,Win};

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

const LEFT_LEG:&str= "
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
pub fn draw(game_state: State) {
    match game_state {
        Win(x) => { match x {
            0 => { println!("{}", BASE) }
            1 => { println!("{}", HEAD) }
            2 => { println!("{}", NECK) }
            3 => { println!("{}", LEFT_ARM) }
            4 => { println!("{}", RIGHT_ARM) }
            5 => { println!("{}", TORSO) }
            6 => { println!("{}", LEFT_LEG) }
            _ => { println!("{}", GAME_OVER) }
        }}
        Lose => { println!("{}", GAME_OVER) }
        Active(n) => match n {
            0 => { println!("{}", BASE) }
            1 => { println!("{}", HEAD) }
            2 => { println!("{}", NECK) }
            3 => { println!("{}", LEFT_ARM) }
            4 => { println!("{}", RIGHT_ARM) }
            5 => { println!("{}", TORSO) }
            6 => { println!("{}", LEFT_LEG) }
            _ => { println!("{}", GAME_OVER) }
        }
    }
}

