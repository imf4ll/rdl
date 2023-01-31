use reqwest::blocking::get;
use crate::utils::types::Format;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PinterestQuality {
    url: String,
    width: u32,
    height: u32,
}

pub fn get_video(url: String) -> (Vec<Format>, String) {
    let res = get(url)
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse video information");

    let video_title = &res
        .split("<title>").collect::<Vec<&str>>()[1]
        .split("</title>").collect::<Vec<&str>>()[0];

    let quality: PinterestQuality = serde_json::from_str(
        format!("{}}}",
            &res
                .split("\"V_720P\":")
                .collect::<Vec<&str>>()[1]
                .split("},\"")
                .collect::<Vec<&str>>()[0]
        )
            .as_str()
    )
        .expect("Failed to parse JSON");

    let mut qualities: Vec<Format> = vec![];
    
    qualities.push(Format {
        url: quality.url,
        quality: format!("{}x{}", quality.width, quality.height),
        audio: String::from(""),
    });

    return (qualities, video_title.to_string());
}
