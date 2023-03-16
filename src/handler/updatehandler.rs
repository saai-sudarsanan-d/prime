use std::io::{Error,ErrorKind};
use crate::task::{Task,getroot, writetask};
use crate::{args::UpdateArgs, parser::parsedeadline, validation::validate_priority};
use std::path::Path;
use std::fs;

pub fn handle(args: UpdateArgs) -> Result<(),Error> {    
    let root = getroot();
    let filenameold = format!("{}/{}.yaml", &root, &args.task_name);
    if !Path::new(&filenameold).is_file() {
        return Err(Error::new(ErrorKind::Other,format!("No such task : {}",&args.task_name)));
    }
    let task_contents = fs::read_to_string(&filenameold).unwrap();
    let mut task:Task = serde_yaml::from_str(&task_contents).unwrap();
    
    match &args.new_task_name {
        Some(t) => {
            let filenamenew = format!("{}/{}.yaml", &root, &t);
            if Path::new(&filenamenew).is_file() {
                return Err(Error::new(ErrorKind::Other,format!("Another task exists with same name")));
            }
            println!("Changing title {} to {}",task.title,&t);
            task.title = &t;
            fs::rename(filenameold, filenamenew).unwrap();
        },
        None => println!("no new name specified")
    }
    
    let nd:Option<String> = match &args.new_deadline {
        Some(d) => {
            let nd = parsedeadline(&d).expect("Invalid Deadline").to_string();
            Some(nd)
        },
        None => None
    };
    
    match &nd {
        Some(d) => task.deadline = &d,
        None => println!("No new deadline specified")
    }

    match args.new_priority {
        Some(p) => {
            println!("Changing priority {} to {}",task.priority,validate_priority(p).expect("Invalid Priority"));
            task.priority = p;
        },
        None => println!("no new priority specified")
    }
    println!("{:?}",task);
    writetask(task.title, task.deadline, task.priority);
    Ok(())
}
