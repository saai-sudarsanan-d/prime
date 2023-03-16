use clap::Parser;
use prime::args::{Mode, PrimeArgs};
use prime::handler::{createhandler, deletehandler, readhandler, updatehandler};

fn main() {
    let opts = PrimeArgs::parse();
    match opts.command {
        Mode::Create(args) => {
            createhandler::handle(args).unwrap();
        }
        Mode::Read(args) => {
            readhandler::handle(args);
        }
        Mode::Update(args) => {
            updatehandler::handle(args).unwrap();
        }
        Mode::Delete(args) => {
            deletehandler::handle(args);
        }
    }
}
