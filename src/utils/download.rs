use colorism::{foreground::Fore, util::RESET};
use reqwest::blocking::get;
use std::io::Write;
use std::fs::File;

pub fn download(url: String) {
    let video_req = get(url).expect("Failed to request video binary");
    let mut file = File::create("video.mp4").expect("Failed to create file");

    file.write_all(&video_req.bytes().expect("Failed to parse video as bytes")).expect("Failed to write file");

    println!("\n{}>>> Video downloaded succesfully!{}", Fore::color(Fore::BdGreen), RESET);
}
