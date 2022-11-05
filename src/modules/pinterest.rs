use reqwest::blocking::get;
use crate::utils::types::Format;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PinterestQuality {
    url: String,
    width: u32,
    height: u32,
}

pub fn get_video(url: String) -> Vec<Format> {
    let res = get(url)
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse video information");

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

    qualities
}
