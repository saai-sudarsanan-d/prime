use crate::{args::CreateArgs, validation::validate_priority,parser::parsedeadline};
use serde::{Serialize,Deserialize};
use std::fs::{DirBuilder,write};
use std::path::Path;
use std::env;

#[derive(Serialize,Deserialize)]
struct NewTask{
    title: String,
    deadline: String,
    priority: u8
}

pub fn handle(args: CreateArgs){
    let root = env::var("PRIME_ROOT").expect("PRIME_ROOT is not set!");
    if !Path::new(&root).is_dir() {
        DirBuilder::new()
        .create(&root)
        .expect("PRIME_ROOT Directory does not exist and could not be created!");
    }
    let filename = format!("{}/{}.yaml",&root,args.task_name);
    if Path::new(&filename).is_file(){
        panic!("Please try a different task name");
    }
    write(
        filename,
        serde_yaml
        ::to_string(&NewTask{
            title:args.task_name,
            deadline:parsedeadline(&args.deadline).expect("Invalid Deadline").to_string(),
            priority:validate_priority(args.priority).expect("Invalid Priority")
        }).expect("Serde YAML Error"))
        .expect("Task could not be created");
}