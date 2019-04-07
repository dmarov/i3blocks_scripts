extern crate chrono;
extern crate serde_json;
extern crate clap;

use chrono::{FixedOffset, Utc, Local };
use clap::{App, Arg, SubCommand, ArgMatches};
use crate::lib::core::Command;

#[derive(Clone)]
pub struct Date<'a,'b> {
    app: App<'a,'b>,
}

impl<'a,'b> Command<'a,'b> for Date<'a,'b> {

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

        Self {
            app: app
        }
    }

    fn get_app(&self) -> &App<'a,'b> {

        &self.app
    }

    fn execute(&self, args: ArgMatches) -> Result<String, Box<dyn std::error::Error>> {

        let format = args.value_of("format")
            .unwrap_or("%d/%m %H:%M");

        let mut date_str;

        if args.is_present("utc") {

            let utc_offset: i32 = args.value_of("utc")
                .unwrap_or("0")
                .parse()
                .unwrap();

            let date_no_tz = Utc::now();
            let date_fixed = FixedOffset::east(utc_offset);
            let date = date_no_tz.with_timezone(&date_fixed);
            date_str = date.format(format).to_string();

        } else {

            let date = Local::now();
            date_str = date.to_string();
        }


        let json = serde_json::json!({
            "version": 1,
            "full_text": date_str,
        });

        Ok(json.to_string())
    }
}
