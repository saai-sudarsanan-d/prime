use crate::task::writetask;
use crate::{args::CreateArgs, parser::parsedeadline, validation::{validate_priority,validate_title}};
use std::io::{Error};

pub fn handle(args: CreateArgs) -> Result<(), Error> {
    let priority = validate_priority(args.priority).expect("Invalid Priority");
    let deadline = parsedeadline(&args.deadline)
        .expect("Invalid Deadline")
        .to_string();
    let task_name = validate_title(&args.task_name).expect("Invalid Task Name");
    writetask(task_name, deadline, priority);
    Ok(())
}
