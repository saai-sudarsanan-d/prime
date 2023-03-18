use clap::Parser;
use prime::args::{Mode, PrimeArgs};
use prime::handler::{createhandler, deletehandler, readhandler, updatehandler};
use std::process;

fn main() {
    let opts = PrimeArgs::parse();
    match opts.command {
        Mode::Create(args) => {
            match createhandler::handle(args) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
        }
        Mode::Read(args) => {
            readhandler::handle(args);
        }
        Mode::Update(args) => {
            match updatehandler::handle(args){
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
        }
        Mode::Delete(args) => {
            match deletehandler::handle(args) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}",e);
                    process::exit(1);
                }
            };
        }
    }
}
