#[derive(Debug, PartialEq)]
pub enum State {
    Win(u8),
    Lose,
    Active(u8), // Contains the number of incorrect guesses
}

//TODO add fields which are generics for handling things like getting input, and displaying to
// abstract those details. The main.rs file can determine which implementation to use.
pub struct HangmanGame {
    word: String,
    displayed_word: Vec<char>,
    incorrect_guesses: Vec<char>,
    max_number_of_guesses: u8,
}

impl HangmanGame {
    pub fn new(word: String, max_number_of_guesses: u8) -> HangmanGame {
        let length = word.len();
        HangmanGame {
            word,
            displayed_word: vec!['_'; length],
            incorrect_guesses: vec![],
            max_number_of_guesses,
        }
    }

    pub fn guess(&mut self, letter: &char) {
        let mut matched = false;
        for index_char in self.word.char_indices() {
            if index_char.1 == *letter {
                self.displayed_word[index_char.0] = *letter;
                matched = true;
            }
        }

        if !matched {
            self.incorrect_guesses.push(*letter);
        }
    }

    pub fn display_word(&self) -> String {
        self.displayed_word.clone().into_iter().map(|c| {
            let mut s = c.to_string();
            s.push(' ');
            s
        }).fold(String::from(""), |mut acc, line| {
            acc.push_str(line.as_str());
            acc
        })
    }

    pub fn incorrect_guesses(&self) -> String {
        self.incorrect_guesses.clone().into_iter().collect()
    }

    pub fn get_state(&self) -> State {
        let guesses: u8 = self.incorrect_guesses.len() as u8;
        if !self.displayed_word.contains(&'_') {
            State::Win(guesses)
        } else if guesses >= self.max_number_of_guesses {
            State::Lose
        } else {
            State::Active(guesses)
        }
    }
}
