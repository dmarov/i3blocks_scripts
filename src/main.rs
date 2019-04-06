mod lib;

use lib::{Executor, commands::Date};

fn main() -> Result<(), std::io::Error> {

    let mut executor = Executor::new();

    executor.add(Date::new());

    let command = executor.command_from_args();

    match command {
        Some(cmd) => {
            let res = cmd.execute();
            println!("{}", res);
        },
        None => {

        }
    }

    Ok(())
}
