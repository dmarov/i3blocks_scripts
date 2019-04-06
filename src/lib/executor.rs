mod executor;

extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};

mod date;

struct Executor {

    pub App: app,
    pub Vec<Command>: commands,
};

impl Executor {

    fn new() {

        let date = Date::new();

        let mut app = App::new("i3blocks scripts")
            .version("0.0.1")
            .author("Dmitry Marov <d.marov94@gmail.com>")
            .about("outputs i3blocks formated data")
            .setting(AppSettings::SubcommandRequired);

        for let command in self.commands {
            app = app.subcommand(date::clap())
        }

        self.app = app;
    }

    fn command_from_args() {
        
        let matches = self.app.get_matches();
        
    }

    fn add(Command: cmd) {
    
    }

}
