use std::process::Command;
use std::io;

use crate::logger;

pub fn convert(video: &String, audio: &String, filename: &String) -> io::Result<()> {
    Command::new("ffmpeg")
        .args(["-i", video, "-i", audio, "-y", filename])
        .spawn()?
        .wait()?;

    println!();
    logger::success(&format!("Converted successfully to {filename}"));

    Ok(())
}
