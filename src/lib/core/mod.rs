extern crate clap;

use clap::{ App, ArgMatches };

pub trait Command<'a,'b> {

    fn new() -> Self;
    fn get_app(&self) -> &App<'a,'b>;
    fn execute(&self, args: ArgMatches) -> Result<String, Box<dyn std::error::Error>>;
}
