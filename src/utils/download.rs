use std::io::Write;
use std::fs::File;
use std::thread;
use std::process::exit;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use crate::logger;

pub fn download(url: String, filename: String) {
    let video_req = get(url)
        .expect("Failed to request video binary");
        
    let total_size = video_req
        .content_length()
        .expect("Failed to get content length");

    println!();

    let pb = ProgressBar::new(total_size);

    pb
        .set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} | {bytes}/{total_bytes} {bar:40.green/white} ({eta})")
        .progress_chars("▉>."));

    let mut file = File::create(&filename)
        .expect("Failed to create file");

    let file_size = get_file_size(&file);

    thread::spawn(move || {
        file
            .write_all(&video_req.bytes().expect("Failed to parse video as bytes"))
            .expect("Failed to write file");
    });

    while file_size < total_size {
        let file_size = get_file_size(&File::open(&filename).expect("Failed to open file"));

        pb.set_position(file_size);

        if file_size == total_size {
            pb.finish();
   
            println!();
            logger::success(&format!("Downloaded \"{filename}\" successfully"));
            
            exit(3);
        }
    };
}

fn get_file_size(file: &File) -> u64 {
    file
        .metadata()
        .unwrap()
        .len()
}
