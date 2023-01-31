use crate::utils::types::Format;
use reqwest::blocking::Client;
use serde::Deserialize;
use crate::logger;
use serde_json;

#[derive(Debug, Deserialize)]
struct TempFormat {
    base_url: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
pub struct StandardFormat {
    playable_url: String,
    playable_url_quality_hd: String,
}

pub fn get_video(url: String) -> (Vec<Format>, String) {
    let formats_req = Client::new();

    let formats_res = formats_req
        .get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36")
        .send()
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse information");

    let video_title = &formats_res
        .split("\"message\":{\"ranges\":[],\"text\":\"").collect::<Vec<&str>>()[1]
        .split("\",\"delight").collect::<Vec<&str>>()[0];

    let mut qualities: Vec<Format> = vec![];

    if formats_res.contains("\"representations\":") {
        let formats: Vec<TempFormat> = serde_json::from_str(
            &formats_res
                .split("\"representations\":").collect::<Vec<&str>>()[1]
                .split(",\"video_id\"").collect::<Vec<&str>>()[0]
        ).unwrap();

        let formats_with_audio: StandardFormat = serde_json::from_str(
            &format!("{{{}}}", &formats_res
                .split("init\":null,").collect::<Vec<&str>>()[1]
                .split(",\"spherical_").collect::<Vec<&str>>()[0])
            ).unwrap();

        qualities.push(Format {
            quality: String::from("SD w/ audio"),
            url: formats_with_audio.playable_url,
            audio: String::from(""),
        });
        
        qualities.push(Format {
            quality: String::from("HD w/ audio"),
            url: formats_with_audio.playable_url_quality_hd,
            audio: String::from(""),
        });

        for format in formats {
            qualities.push(Format {
                url: format.base_url,
                quality: format!("{}x{}", format.width, format.height),
                audio: String::from(""),
            });
        }

    } else if formats_res.contains("\"playable_url\":") {
        let format_sd = &formats_res
            .split("\"playable_url\":\"").collect::<Vec<&str>>()[1]
            .split("\",\"playable_url").collect::<Vec<&str>>()[0];

        qualities.push(Format {
            url: format_sd.to_string().replace("\\", ""),
            quality: String::from("SD"),
            audio: String::from(""),
        });

        let format_hd = &formats_res
            .split("\"playable_url_quality_hd\":\"").collect::<Vec<&str>>()[1]
            .split("\",\"spherical").collect::<Vec<&str>>()[0];

        qualities.push(Format {
            url: format_hd.to_string().replace("\\", ""),
            quality: String::from("HD"),
            audio: String::from(""),
        });

    } else {
        logger::error("Invalid video parsed");

    }

    return (qualities, video_title.to_string());
}
