pub mod core;
pub mod commands;

extern crate clap;
use clap::{App, AppSettings};
use self::core::Command;
use std::error;

pub struct Executor<'a,'b,T> 
    where T: Command<'a,'b>
{
    app: App<'a,'b>,
    commands: Vec<T>,
}

impl<'a,'b,T> Executor<'a,'b,T> 
    where T: Command<'a,'b>
{

    pub fn new() -> Self {

        let app = App::new("i3blocks scripts")
            .version("0.0.1")
            .author("Dmitry Marov <d.marov94@gmail.com>")
            .about("outputs i3blocks formated data")
            .setting(AppSettings::SubcommandRequired);

        let commands: Vec<T> = Vec::new();

        Self {
            app: app,
            commands: commands,
        }
    }

    pub fn perform(&self) -> Result<String, Box<dyn error::Error>> {

        let app = &self.app;
        let matches = app.clone().get_matches();
        let name = matches.subcommand_name().unwrap();

        let commands = &self.commands;
        let cmd = commands
            .into_iter()
            .find(|&item| item.get_app().clone().get_name() == name)
            .unwrap();

        cmd.execute(matches)
    }

    pub fn add(&mut self, cmd: T) {

        self.app = self.app.clone().subcommand(cmd.get_app().clone());
        self.commands.push(cmd);
    }
}
