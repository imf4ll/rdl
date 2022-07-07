mod modules;
mod utils;
mod uses;

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        uses::twitter::get(args[1].to_string());
    
    } else {
        panic!("Specify at least a URL");

    }
}
