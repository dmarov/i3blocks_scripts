extern crate chrono;
extern crate serde_json;
extern crate clap;

use chrono::{FixedOffset, Utc };
use clap::{App, Arg, SubCommand};

struct Date {
    app: App,
}

impl Command for Date {

    fn new(&self) -> Self {

        self.app = SubCommand::with_name(self.name)
            .about("return current date")
            .version("0.1.0")
            .arg(
                Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .value_name("STRFTIME_FMT")
                    .help("Sets a custom date format")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("utc")
                    .short("u")
                    .long("utc")
                    .value_name("UTC_OFFSET")
                    .help("Sets a custom utc offset")
                    .takes_value(true)
            );
    }

    fn execute(&self) {

        let matches = self.app.get_matches();

        if let Some(matches) = matches.subcommand_matches("date") {

            let format = matches.value_of("format")
                .unwrap_or("%d/%m %H:%M");

            println!("{}", format);
        }

        // let res = process_command();
        // print!("{}", res);

        // let args: Vec<String> = std::env::args().collect();

        // let fmt_str = &args[1];
        // let offset = &args[2];
        // let offset_sec: i32 = offset.parse().unwrap();

        // let date = Utc::now();
        // let date_fixed = FixedOffset::east(offset_sec);
        // let date_tz = date.with_timezone(&date_fixed);
        // let date_str = date_tz.format(fmt_str).to_string();

        // let json = serde_json::json!({
        //     "version": 1,
        //     "full_text": date_str,
        // });

        result
    }
}
