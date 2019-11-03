pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub enum InputOutput {
    StdIn,
    File,
    GUI,
}

pub enum Database {
    File,
    Redis,
}

// TODO A cool feature to add would be two player support.
pub struct Configuration {
    difficulty: Difficulty,
    input: InputOutput,
    output: InputOutput,
    database: Database,
}

impl Configuration {
    pub fn new(
        difficulty: Difficulty,
        input: InputOutput,
        output: InputOutput,
        database: Database,
    ) -> Configuration {
        Configuration {
            difficulty,
            input,
            output,
            database,
        }
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        &self.difficulty
    }
    pub fn get_input(&self) -> &InputOutput {
        &self.output
    }
    pub fn get_output(&self) -> &InputOutput {
        &self.output
    }
}
