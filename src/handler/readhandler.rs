use crate::{args::ReadArgs, task::{Task,getroot}};
use glob::glob;
use std::fs;
use colored::Colorize;

pub fn handle(_args: ReadArgs){
    let root= getroot();
    let entries:Vec<_> = glob(&format!("{}/*.yaml",&root)).unwrap().collect();
    for entry in entries{ 
        let filename = entry.unwrap().to_str().unwrap().to_owned();
        let task_contents = fs::read_to_string(&filename).unwrap();
        let task:Task = serde_yaml::from_str(&task_contents).unwrap();
        println!("{}",task.title.red().bold());
        println!("{}",task.priority.to_string().blue());
        println!("{}",task.deadline.green());
    }
}

// Let's Get Rusty
// Rust Lang Book