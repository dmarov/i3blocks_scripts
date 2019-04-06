extern crate clap;

use clap::SubCommand;

trait Command {

    fn new() -> Self;

    fn clap(&self) -> SubCommand;

    fn execute(&self) {
    
    }

    fn get_name() -> String;
};



