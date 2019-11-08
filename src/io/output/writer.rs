use crate::hangman::State;
pub trait Writer {
    fn write_game_state(&self, game_state: State, display_word: String, incorrect_guesses: String);
}
