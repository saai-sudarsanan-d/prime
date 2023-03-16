use crate::task::{getroot, writetask};
use crate::{args::CreateArgs, parser::parsedeadline, validation::validate_priority};
use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn handle(args: CreateArgs) -> Result<(), Error> {
    let priority = validate_priority(args.priority).expect("Invalid Priority");
    let deadline = parsedeadline(&args.deadline)
        .expect("Invalid Deadline")
        .to_string();

    // Creating a Task
    let root = getroot();
    let filename = format!("{}/{}.yaml", &root, &args.task_name);
    if Path::new(&filename).is_file() {
        return Err(Error::new(
            ErrorKind::Other,
            "Task with same name already exists",
        ));
    }
    writetask(&args.task_name, &deadline, priority);
    Ok(())
}
