mod lib;

use lib::{Executor, commands::Date};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut executor = Executor::new();

    executor.add(Date::new());

    let command = executor.command_from_args();

    match command {
        Some(cmd) => {
            let res = cmd.execute()
                .unwrap()
                .expect("execution error");

            println!("{}", res);
            Ok(())
        },
        None => {
            std::io::Error::new("command not found");
        },
    }
}
