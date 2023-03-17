use crate::args::ReadArgs;

pub fn handle(args: ReadArgs) -> bool {
    println!("{:?}", args);
    
    true
}