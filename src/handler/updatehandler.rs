use std::io::{Error,ErrorKind};
use crate::task::{Task,getroot,writetask, taskexists};
use crate::validation::validate_title;
use crate::{args::UpdateArgs, parser::parsedeadline, validation::validate_priority};
use std::fs;
use std::process;

pub fn handle(args: UpdateArgs) -> Result<(),Error> {
    if taskexists(&args.task_name) {
        let root = getroot();
        let filenameold = format!("{}/{}.yaml",&root,&args.task_name);
        let task_contents = match fs::read_to_string(&filenameold) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}",e);
                process::exit(1);
            }
        };
        let mut task:Task = match serde_yaml::from_str(&task_contents) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}",e);
                process::exit(1);
            }
        };
        
        match &args.new_task_name {
            Some(t) => {
                println!("Changing title {} to {}",task.title,t);
                task.title = match validate_title(&t){ 
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                };
                match fs::rename(filenameold, format!("{}/{}.yaml", &root, &t)) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                };
            },
            None => println!("no new name specified")
        }
        
        let nd:Option<String> = match args.new_deadline {
            Some(d) => {
                let nd = match parsedeadline(&d){
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                }.to_string();
                Some(nd)
            },
            None => None
        };
        
        match nd {
            Some(d) => {
                println!("Changing deadline {} to {}",task.deadline,d);
                task.deadline = d;
            },
            None => println!("No new deadline specified")
        }

        match args.new_priority {
            Some(p) => {
                println!("Changing priority {} to {}",task.priority,match validate_priority(p) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                });
                task.priority = p;
            },
            None => println!("no new priority specified")
        }
        writetask(task.title, task.deadline, task.priority);
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other,"No such task exists!"))
    }
}
