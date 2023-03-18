use std::fs;
use std::io::{Error,ErrorKind};
use crate::{args::DeleteArgs, task::{getroot, taskexists}};
use std::process;
pub fn handle(args: DeleteArgs) -> Result<(),Error> {
    let root = getroot();
    if taskexists(&args.task_name){
        let oldfile = format!("{}/{}.yaml",&root,&args.task_name);
        let newfile;
        if args.archive {
            match fs::create_dir_all(&format!("{}/archive/complete",&root)){
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
           match fs::create_dir_all(&format!("{}/archive/incomplete",&root)){
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
            if args.done {
                newfile = format!("{}/archive/complete/{}.yaml",&root,&args.task_name);
            }
            else {
                newfile = format!("{}/archive/incomplete/{}.yaml",&root,&args.task_name);
            }
           match fs::rename(oldfile, newfile){
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
        } else {
           match fs::remove_file(oldfile){
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
        }
        Ok(())
    }
    else {
        Err(Error::new(ErrorKind::Other,"No such task exists!"))
    }
}
