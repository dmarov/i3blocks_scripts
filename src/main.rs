mod lib;

use lib::{Executor, commands::Date, core::Command};
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let executor = Executor::new();

    executor.add(Date::new());

    let command = executor.command_from_args();

    match command {
        Some(cmd) => {
            let res = cmd.execute()
                .expect("execution error");

            println!("{}", res);
            Ok(())
        },
        None => {
            Err(Box::new(Error::new(ErrorKind::Other, "command not found")))
        },
    }
}
