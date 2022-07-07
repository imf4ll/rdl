mod modules;
mod utils;
mod uses;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    url: String,

    #[clap(short, long, value_parser, default_value = "video.mp4")]
    filename: String,
}

fn main() {
    let args = Args::parse();

    if args.url == "" {
        panic!("Invalid URL provided");
    
    } else {
        if args.url.contains("twitter") {
            uses::twitter::get(args.url, args.filename);

        } else {
            panic!("Invalid URL provided");

        }
    }
}
