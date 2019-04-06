pub trait Command {

    fn new() -> Self;
    fn execute(&self) -> Result<String, Box<dyn std::error::Error>>;
}
