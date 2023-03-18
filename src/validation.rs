use std::io::{Error, ErrorKind};
use crate::task::getroot;
use std::path::Path;

use regex::Regex;
pub fn validate_deadline(deadline: &str) -> Result<&str, Error> {
    let dl_re = Regex::new(r"^\d{1,3}[\-][wdhms]$").unwrap();
    let date_re = Regex::new(r"^((18|19|20)[0-9]{2}[\-.](0[13578]|1[02])[\-.](0[1-9]|[12][0-9]|3[01]))|(18|19|20)[0-9]{2}[\-.](0[469]|11)[\-.](0[1-9]|[12][0-9]|30)|(18|19|20)[0-9]{2}[\-.](02)[\-.](0[1-9]|1[0-9]|2[0-8])|(((18|19|20)(04|08|[2468][048]|[13579][26]))|2000)[\-.](02)[\-.]29$").unwrap();
    if dl_re.is_match(deadline) {
        Ok("SHORT")
    } else if date_re.is_match(deadline) {
        Ok("LONG")
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Please provide a proper deadline (--help for more details)",
        ))
    }
}

pub fn validate_priority(priority: u8) -> Result<u8, Error> {
    if priority < 4 {
        Ok(priority)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Priority must be greater than equal to 0 and less than 4!",
        ))
    }
}

pub fn validate_title(task_name: &str) -> Result<String, Error> {
    let root = getroot();
    let filename = format!("{}/{}.yaml", root, task_name);
    if Path::new(&filename).is_file() {
        return Err(Error::new(
            ErrorKind::Other,
            "Task with same name already exists",
        ));
    }
    Ok(String::from(task_name))
}
