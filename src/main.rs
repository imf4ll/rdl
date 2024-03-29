mod modules;
mod utils;
mod logger;
mod config;
mod get;

use utils::types::Format;
use crate::get::get;
use crate::modules::*;
use clap::Parser;

#[derive(Parser)]
#[clap(version, about = "A tool to download videos from some places")]
struct Args {
    /// Video URL
    #[clap(short, long, value_parser)]
    url: String,

    /// Filename or path to save
    #[clap(short, long, value_parser, default_value = "video.mp4")]
    filename: String,

    /// Enable fast mode (skip format choosing)
    #[clap(long, action)]
    fast: bool,
}

fn main() {
    let mut args = Args::parse();
    let mut qualities: Vec<Format> = vec![];
    let mut video_title: String = String::from("");

    let config = config::parse();

    if config.path != "" && args.filename != "video.mp4" {
        args.filename = format!("{}/{}", config.path, args.filename);

    } else if config.path != "" {
        args.filename = format!("{}/video.mp4", config.path);

    }
    
    if args.url.contains("twitter") {
        (qualities, video_title) = twitter::get_video(args.url);

    } else if args.url.contains("facebook") {
        (qualities, video_title) = facebook::get_video(args.url);

    } else if args.url.contains("rumble") {
        (qualities, video_title) = rumble::get_video(args.url);

    } else if args.url.contains("reddit") {
        (qualities, video_title) = reddit::get_video(args.url);

    } else if args.url.contains("pinterest") {
        (qualities, video_title) = pinterest::get_video(args.url);

    } else {
        logger::error("Invalid URL provided");

    }

    get(qualities, video_title, args.filename, args.fast);
}
