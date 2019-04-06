pub trait Command {

    fn new(&self) -> Self;
    fn execute(&self) -> Result<String, Box<dyn std::error::Error>>;
}
