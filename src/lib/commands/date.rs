extern crate chrono;
extern crate serde_json;
extern crate clap;

use chrono::{FixedOffset, Utc, Local };
use clap::{App, Arg, SubCommand};
use crate::lib::core::Command;

pub struct Date<'a,'b> {
    app: App<'a,'b>,
}

impl<'a,'b> Command for Date<'a,'b> {

    fn new() -> Self {

        let app = SubCommand::with_name("date")
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

        Self(app)
    }

    fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {

        let matches = self.app.get_matches();

        let format = matches.value_of("format")
            .unwrap_or("%d/%m %H:%M");

        let mut date = Local::now();

        if matches.is_present("utc") {

            let utc_offset = matches.value_of("utc").unwrap();
            let date_no_tz = Utc::now();
            let date_fixed = FixedOffset::east(utc_offset);
            date = date_no_tz.with_timezone(&date_fixed);
        }

        let date_str = date.format(format).to_string();

        let json = serde_json::json!({
            "version": 1,
            "full_text": date_str,
        });

        json
    }
}
