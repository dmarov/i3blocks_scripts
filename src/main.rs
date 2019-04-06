mod lib;

use lib::commands;

fn main() -> Result<(), std::io::Error> {

    let mut executor = lib::executor::new();

    executor.add(commands::Date::new())
    executor.add(commands::Temperature::new())

    let command = executor.command_from_args();
    let result = command.process();

    println!("{}", result);

    Ok(())
}
