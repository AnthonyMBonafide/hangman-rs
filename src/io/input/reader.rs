pub trait Reader {
    fn read_guess(&self) -> Result<char, String>;
}
