use crate::{args::ReadArgs, task::{Task,getroot}, parser::parsedeadline,validation::has_spl_chars};
use chrono::{DateTime,Local};
use glob::glob;
use std::fs;
use colored::Colorize;
use regex::Regex;

pub fn handle(args: ReadArgs){
    let root= getroot();
    let entries:Vec<_> = glob(&format!("{}/*.yaml",&root)).unwrap().collect();
    let mut count = 0;
    for entry in entries{ 
        let filename = entry.unwrap().to_str().unwrap().to_owned();
        let task_contents = fs::read_to_string(&filename).unwrap();
        let task:Task = serde_yaml::from_str(&task_contents).unwrap();

        let mut tshow = false;
        let mut dshow = false;
        let mut pshow = false;

        // title check
        match &args.task_name { 
            Some(q) => {
                if has_spl_chars(&q){
                    panic!("query string cannot contain special characters");
                }
                let query = Regex::new(&format!(r"{}",q)).unwrap();
                if query.is_match(&task.title) {
                    tshow = true;                
                }
            },
            None => {tshow = true;}
            
        }
        
        // deadline check
        match &args.deadline { 
            Some(d) => {
                let dl = parsedeadline(d).unwrap();
                if task.deadline.parse::<DateTime<Local>>().unwrap() < dl {
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