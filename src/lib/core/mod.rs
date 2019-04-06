pub trait Command {

    fn new(&self) -> Self;
    fn execute(&self) -> Result<String, std::error::Error>;
}
