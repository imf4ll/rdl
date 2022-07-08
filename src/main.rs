mod modules;
mod utils;
mod uses;
mod logger;

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

    if args.filename == "video.mp4" {
        logger::warn("Filename not provided, using default \"video.mp4\"");

    }

    if args.url.contains("twitter") {
        uses::twitter::get(args.url, args.filename);

    } else {
        logger::error("Invalid URL provided");

    }
}
