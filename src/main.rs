use clap::Parser;
use prime::args::{PrimeArgs,Mode};
use prime::handler::{createhandler,readhandler,updatehandler,deletehandler};

fn main(){
    let opts = PrimeArgs::parse();
    match opts.command {
        Mode::Create(args) => {createhandler::handle(args);},
        Mode::Read(args) => {readhandler::handle(args);},
        Mode::Update(args) => {updatehandler::handle(args);},
        Mode::Delete(args) => {deletehandler::handle(args);}
    }
}
