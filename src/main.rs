mod lib;

use lib::{Executor, commands::Date, core::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut executor = Executor::new();

    executor.add(Date::new());

    let result = executor.perform();

    match result {
        Ok(result_ok) => {

            println!("{}", result_ok);
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }
}
