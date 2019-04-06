pub mod core;
pub mod commands;

extern crate clap;
use clap::{App, AppSettings};
use self::core::Command;

pub struct Executor<'a,'b,T> 
    where T: Command
{

    pub app: App<'a,'b>,
    commands: Vec<T>,
}

impl<'a,'b,T> Executor<'a,'b,T> 
    where T: Command
{

    pub fn new() -> Self {

        let app = App::new("i3blocks scripts")
            .version("0.0.1")
            .author("Dmitry Marov <d.marov94@gmail.com>")
            .about("outputs i3blocks formated data")
            .setting(AppSettings::SubcommandRequired);

        let commands: Vec<T> = Vec::new();

        Executor{
            app: app,
            commands: commands,
        }
    }

    pub fn command_from_args(&self) -> Option<Box<T>> {

        let name = self.app.get_name();

        let mut res = None;
        for cmd in self.commands {

            if cmd.app.get_name() == name {

                res = Some(cmd);
                break;
            }

            Some(Box::new(res))
        }

        res
    }

    pub fn add(&self, cmd: T) {

        self.commands.push(cmd);
        self.app.subcommand(cmd.app);
    }
}
