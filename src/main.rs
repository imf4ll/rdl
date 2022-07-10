mod modules;
mod utils;
mod uses;
mod logger;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about = "A tool to download videos from some places")]
struct Args {
    #[clap(short, long, value_parser)]
    url: String,

    #[clap(short, long, value_parser, default_value = "video.mp4")]
    filename: String,
}

fn main() {
    let mut args = Args::parse();
    
    let config = utils::config::parser();

    if config.path != "" && args.filename != "video.mp4" {
        args.filename = format!("{}/{}", config.path, args.filename);

    } else if config.path != "" {
        args.filename = format!("{}/video.mp4", config.path);

    }
    
    if args.url.contains("twitter") {
        uses::twitter::get(args.url, args.filename);

    } else if args.url.contains("facebook") {
        uses::facebook::get(args.url, args.filename);

    } else if args.url.contains("rumble") {
        uses::rumble::get(args.url, args.filename)

    } else {
        logger::error("Invalid URL provided");

    }
}
