use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{write, DirBuilder};
use std::path::Path;

#[derive(Serialize, Deserialize,Debug)]
pub struct Task<'a> {
    pub title: &'a str,
    pub deadline: &'a str,
    pub priority: u8,
}

pub fn getroot() -> String {
    let root = env::var("PRIME_ROOT").expect("PRIME_ROOT is not set!");
    root
}

pub fn checkandcreateroot() {
    let root = getroot();
    if !Path::new(&root).is_dir() {
        DirBuilder::new()
            .create(&root)
            .expect("PRIME_ROOT Directory does not exist and could not be created!");
    }
}

// pub fn readtask(task_name: &str){
// }

pub fn writetask(task_name: &str, deadline: &str, priority: u8) {
    let root = getroot();
    checkandcreateroot();
    let filename = format!("{}/{}.yaml", &root, task_name);
    write(
        filename,
        serde_yaml::to_string(&Task {
            title: task_name,
            deadline,
            priority,
        })
        .expect("Serde YAML Error"),
    )
    .expect("Task could not be created");
}
