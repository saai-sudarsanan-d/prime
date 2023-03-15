use chrono::{DateTime,Local,Duration};
use std::{io::{Error,ErrorKind}};
use crate::validation::validate_deadline;

pub fn parsedeadline(darg:&str) -> Result<DateTime<Local>,Error>{
    match validate_deadline(darg) {
        Ok(s) => {
            if s == "SHORT" {
                let parts:Vec<&str> = darg.split("-").collect();
                let deadline = match parts[1] {
                    "w" => Local::now() + Duration::weeks(parts[0].parse().unwrap()),
                    "d" => Local::now() + Duration::days(parts[0].parse().unwrap()),
                    "h" => Local::now() + Duration::hours(parts[0].parse().unwrap()),
                    "m" => Local::now() + Duration::minutes(parts[0].parse().unwrap()),
                    "s" => Local::now() + Duration::seconds(parts[0].parse().unwrap()),
                    _ => Local::now()
                };
                println!("{:?}",deadline);
                Ok(deadline)
            } else {
                let deadline:DateTime<Local> = format!("{}T00:00:00+00:00",darg).parse::<DateTime<Local>>().unwrap();
                if deadline < Local::now() {
                    return Err(Error::new(ErrorKind::Other,"Date must be sometime in the future!"))
                }
                Ok(deadline)
            }
        },
        Err(e) => Err(e)
    }
}