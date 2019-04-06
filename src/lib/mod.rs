pub mod core;
pub mod commands;

extern crate clap;
use clap::{App, AppSettings};
use core::Command;

struct Executor {

    app: App,
    commands: Vec<Command>,
}

impl Executor {

    fn new() {

        let app = App::new("i3blocks scripts")
            .version("0.0.1")
            .author("Dmitry Marov <d.marov94@gmail.com>")
            .about("outputs i3blocks formated data")
            .setting(AppSettings::SubcommandRequired);

        self.app = app;
    }

    fn command_from_args() -> Option<Command> {

        match self.app.subcommand_name() {
            Some(name) => {

                let mut res = None;
                for cmd in self.commands {

                    if cmd.app.get_name() == name {

                        res = Some(cmd);
                        break;
                    }
                }

                res
            },
            None => None
        }
    }

    fn add(commands::Command: cmd) {

        self.commands.push(cmd);
        self.app.subcommand(cmd.app);
    }
}
