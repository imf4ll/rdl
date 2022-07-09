use crate::utils::types::Format;
use reqwest::blocking::get;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct TempFormat {
    url: String,
    meta: TempFormatMeta,
}

#[derive(Debug, Deserialize)]
struct TempFormatMeta {
    w: u32,
    h: u32,
}

fn get_video_id(url: String) -> String {
    let request = get(url)
        .expect("Failed to request video ID")
        .text()
        .expect("Failed to parse video ID information");


    let video_id = request
        .split("\"video\":\"").collect::<Vec<&str>>()[1]
        .split("\"").collect::<Vec<&str>>()[0];

    video_id.to_string()
}

pub fn get_video(url: String) -> Vec<Format> {
    let video_response = get(
        format!("https://rumble.com/embedJS/u3/?request=video&ver=2&v={}&ext=%7B%22ad_count%22%3Anull%7D&ad_wt=9043"
            , get_video_id(url))
    )
        .expect("Failed to get video information")
        .text()
        .expect("Failed to parse video information");

    let raw_formats = video_response
        .split("\"ua\":{\"mp4\":{").collect::<Vec<&str>>()[1]
        .split("},\"webm\"").collect::<Vec<&str>>()[0];

    let formats: Vec<TempFormat> = serde_json::from_str(
        &format!("[{}]", raw_formats
            .replace("\"240\":", "")
            .replace("\"360\":", "")
            .replace("\"480\":", "")
            .replace("\"720\":", "")
            .replace("\"1080\":", "")
        )
    ).unwrap();

    let mut qualities: Vec<Format> = vec![];

    for format in formats {
        qualities.push(Format {
            url: format.url,
            quality: format!("{}x{}", format.meta.w, format.meta.h),
        })
    }

    qualities
}
