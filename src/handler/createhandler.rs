use crate::task::writetask;
use crate::{args::CreateArgs, parser::parsedeadline, validation::{validate_priority,validate_title}};
use std::io::{Error};
use std::process;

pub fn handle(args: CreateArgs) -> Result<(), Error> {
    let priority = match validate_priority(args.priority) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}",e);
            process::exit(1);
        }
    };
    let deadline = match parsedeadline(&args.deadline){
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}",e);
            process::exit(1);
        }
    }.to_string();
    let task_name = match validate_title(&args.task_name) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}",e);
            process::exit(1);
        }
    };
    writetask(task_name, deadline, priority);
    Ok(())
}
