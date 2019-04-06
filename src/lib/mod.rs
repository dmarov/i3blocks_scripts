pub mod actions;
pub mod commands;

extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};

struct Executor {

    pub App: app,
    pub Vec<Command>: commands,
};

impl Executor {

    fn new(Vec<Command>: cmds) {

        let mut app = App::new("i3blocks scripts")
            .version("0.0.1")
            .author("Dmitry Marov <d.marov94@gmail.com>")
            .about("outputs i3blocks formated data")
            .setting(AppSettings::SubcommandRequired);

        self.app = app;
    }

    fn command_from_args() {
        
        let matches = self.app.get_matches();
        
    }

    fn add(commands::Command: cmd) {
    
        self.commands.push(cmd);
        self.app.subcommand(cmd.clap());
    }

}
