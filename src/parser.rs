use crate::validation::validate_deadline;
use chrono::{DateTime, Duration, Local};
use std::io::{Error, ErrorKind};
use std::process;
use colored::Colorize;
// Error Handling

pub fn parsedeadline(darg: &str) -> Result<DateTime<Local>, Error> {
    match validate_deadline(darg) {
        Ok(s) => {
            if s == "SHORT" {
                let parts: Vec<&str> = darg.split("-").collect();
                let deadline = match parts[1] {
                    "w" => Local::now() + Duration::weeks(match parts[0].parse() {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Error: {}\n{}",e,String::from("Number of Units of time should be a valid Integer").blue().bold());
                            process::exit(1);
                        }
                    }),
                    "d" => Local::now() + Duration::days(match parts[0].parse(){
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Error: {}\n{}",e,String::from("Number of Units of time should be a valid Integer").blue().bold());
                            process::exit(1);
                        }
                    }),
                    "h" => Local::now() + Duration::hours(match parts[0].parse(){
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Error: {}\n{}",e,String::from("Number of Units of time should be a valid Integer").blue().bold());
                            process::exit(1);
                        }
                    }),
                    "m" => Local::now() + Duration::minutes(match parts[0].parse(){
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Error: {}\n{}",e,String::from("Number of Units of time should be a valid Integer").blue().bold());
                            process::exit(1);
                        }
                    }),
                    "s" => Local::now() + Duration::seconds(match parts[0].parse(){
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Error: {}\n{}",e,String::from("Number of Units of time should be a valid Integer").blue().bold());
                            process::exit(1);
                        }
                    }),
                    _ => Local::now(),
                };
                Ok(deadline)
            } else {
                let deadline: DateTime<Local> = match format!("{}T00:00:00+00:00", darg)
                    .parse::<DateTime<Local>>() {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("{}",e);
                            process::exit(1);
                        }
                    };
                if deadline < Local::now() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Date must be sometime in the future!",
                    ));
                }
                Ok(deadline)
            }
        }
        Err(e) => {
            eprintln!("Error: {}\n{}",e,String::from("You have not entered a proper deadline").blue().bold());
            process::exit(1);
        }
    }
}
