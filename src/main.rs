mod lib;

fn main() -> Result<(), std::io::Error> {

    let executor = lib::executor::new();
    let command = executor.command_from_args();
    let result = command.process();

    println!("{}", result);

    Ok(())
}
