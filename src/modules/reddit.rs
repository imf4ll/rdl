use crate::utils::types::QUALITIES;
use crate::utils::types::Format;
use reqwest::blocking::get;
use crate::logger;

pub fn get_video(url: String) -> Vec<Format> {
    let res = get(format!("{url}.json"))
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse video information");

    if !res.contains("\"url_overridden_by_dest\"") {
        logger::error("Invalid video parsed");

    }

    let video_id = res
        .split("\"url_overridden_by_dest\": \"").collect::<Vec<&str>>()[1]
        .split("\"").collect::<Vec<&str>>()[0]
        .split("/").collect::<Vec<&str>>()[3];

    let quality: u32 = res
        .split("\"fallback_url\": \"").collect::<Vec<&str>>()[1]
        .split("\"").collect::<Vec<&str>>()[0]
        .split("DASH_").collect::<Vec<&str>>()[1]
        .split(".").collect::<Vec<&str>>()[0]
        .parse()
        .expect("Failed to parse quality label");

    let mut qualities: Vec<Format> = vec![];

    for q in QUALITIES {
        if q <= quality {
            qualities.push(Format {
                url: format!("https://v.redd.it/{video_id}/DASH_{q}.mp4"),
                quality: format!("{q}p"),
                audio: format!("https://v.redd.it/{video_id}/DASH_audio.mp4"),
            });
        }
    }

    qualities
}
