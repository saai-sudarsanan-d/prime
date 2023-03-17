use std::fs;
use std::io::{Error,ErrorKind};
use crate::{args::DeleteArgs, task::{getroot, taskexists}};
pub fn handle(args: DeleteArgs) -> Result<(),Error> {
    let root = getroot();
    if taskexists(&args.task_name){
        let oldfile = format!("{}/{}.yaml",&root,&args.task_name);
        let newfile;
        if args.archive {
            fs::create_dir_all(&format!("{}/archive/complete",&root)).unwrap();
            fs::create_dir_all(&format!("{}/archive/incomplete",&root)).unwrap();
            if args.done {
                newfile = format!("{}/archive/complete/{}.yaml",&root,&args.task_name);
            }
            else {
                newfile = format!("{}/archive/incomplete/{}.yaml",&root,&args.task_name);
            }
            fs::rename(oldfile, newfile).unwrap();
        } else {
            fs::remove_file(oldfile).unwrap();
        }
        Ok(())
    }
    else {
        Err(Error::new(ErrorKind::Other,"No such task exists!"))
    }
}
