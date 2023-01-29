use std::io::Write;
use std::fs::File;
use std::thread;
use std::process::{exit, Command};

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{blocking, StatusCode};
use dialoguer::{Select, theme::ColorfulTheme};
use crate::logger;
use crate::utils::types::Format;

pub fn get(qualities: Vec<Format>, filename: String, fast: bool) {
    logger::success(&format!("{} qualities available...\n", qualities.len()));

    if fast {
        download(qualities[0].url.to_string(), filename, qualities[0].audio.to_string());
            
    } else {
        match qualities.len() > 1 {
            true => {
                let choose = &qualities[choose(&qualities)];

                download(choose.url.to_string(), filename, choose.audio.to_string())
            },

            false => download(qualities[0].url.to_string(), filename, qualities[0].audio.to_string()),
        }
    } 
}

fn download(url: String, filename: String, audio: String) {
    if audio == "" {
        with_progress(&url, &filename);

    } else {
        let valid_audio = blocking::get(&audio)
            .expect("Failed to get audio");

        if valid_audio.status() != StatusCode::OK {
            logger::warn("Failed to get audio from video, downloading only the video");

            with_progress(&url, &filename);

        } else {
            convert(&url, &audio, &filename)
                .expect("Failed to convert file");
            
        }
    }

    exit(3);
}

fn with_progress(url: &String, filename: &String) {
    let video_req = blocking::get(url)
        .expect("Failed to request video binary");
        
    let total_size = video_req
        .content_length()
        .expect("Failed to get content length");

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
        let file_size = get_file_size(&File::open(&filename)
            .expect("Failed to open file"));

        pb.set_position(file_size);

        if file_size == total_size {
            pb.finish();
   
            println!();
            logger::success(&format!("Downloaded \"{filename}\" successfully"));

            break;
        }
    }
}

fn choose(qualities: &Vec<Format>) -> usize {
    let mut qualities_with_length_vec: Vec<String> = vec![];

    for q in qualities.iter() {
        let length = blocking::get(&q.url)
            .unwrap()
            .content_length()
            .unwrap();

        qualities_with_length_vec.push(format!("{} ({})", q.quality, size(length)));
    }

    Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&qualities_with_length_vec)
        .interact()
        .unwrap()
}

fn convert(video: &String, audio: &String, filename: &String) -> std::io::Result<()> {
    Command::new("ffmpeg")
        .args(["-i", video, "-i", audio, "-y", filename])
        .spawn()?
        .wait()?;

    println!();
    logger::success(&format!("Converted successfully to {filename}"));

    Ok(())
}

fn get_file_size(file: &File) -> u64 {
    file
        .metadata()
        .unwrap()
        .len()
}

fn size(length: u64) -> String {
    let size: f32 = length as f32 / 1000000.0;

    if size < 1024.0 {
        format!("{:.1} MB", size)
    
    } else {
        format!("{:.1} GB", size)

    }
}
