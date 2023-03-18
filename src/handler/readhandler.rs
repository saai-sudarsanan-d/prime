use crate::{args::ReadArgs, task::{Task,getroot}, parser::parsedeadline,validation::has_spl_chars};
use chrono::{DateTime,Local};
use glob::glob;
use std::fs;
use colored::Colorize;
use regex::Regex;
use std::process;

pub fn handle(args: ReadArgs){
    let root= getroot();
    let entries:Vec<_> = match glob(&format!("{}/*.yaml",&root)){
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            }.collect();
    let mut count = 0;
    for entry in entries{ 
        let filename = 
        match 
            match entry {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            }.to_str() 
        {
            Some(r) => r,
            None => continue
        }.to_owned();

        let task_contents = match fs::read_to_string(&filename) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}",e);
                process::exit(1);
            }
        };
        
        let task:Task = match serde_yaml::from_str(&task_contents) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}",e);
                process::exit(1);
            }
        };

        let mut tshow = false;
        let mut dshow = false;
        let mut pshow = false;

        // title check
        match &args.task_name { 
            Some(q) => {
                if has_spl_chars(&q){
                    eprintln!("{}",String::from("query string cannot contain special characters").red());
                    process::exit(1);
                }

                let query = match Regex::new(&format!(r"{}",q)){
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                };
                
                if query.is_match(&task.title) {
                    tshow = true;                
                }
            },
            None => {tshow = true;}
            
        }
        
        // deadline check
        match &args.deadline { 
            Some(d) => {
                let dl = match parsedeadline(d){
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                };
                if match task.deadline.parse::<DateTime<Local>>() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}",e);
                        process::exit(1);
                    }
                } < dl {
                    dshow = true;
                }            
            },
            None => {dshow = true;}
            
        }

        // priority check
        match args.priority { 
            Some(p) => {
                if task.priority == p{
                    pshow = true;
                }
            },
            None => {pshow = true;}
            
        }

        // count check
        if tshow && pshow && dshow {
            count += 1;
            if count > args.count {
                break
            }
        }
        if tshow && pshow && dshow {
            println!("{}",task.title.red().bold());
            println!("{}",task.priority.to_string().blue());
            println!("{}",task.deadline.green());
        }
    }
}

// Let's Get Rusty
// Rust Lang Book