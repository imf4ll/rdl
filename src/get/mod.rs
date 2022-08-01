mod utils;

use utils::*;
use crate::logger;
use crate::utils::types::Format;

pub fn get(qualities: Vec<Format>, filename: String) {
    logger::success(&format!("{} qualities available...\n", qualities.len()));

    match qualities.len() > 1 {
        true => {
            let choose = &qualities[choose(&qualities)];

            download(choose.url.to_string(), filename, choose.audio.to_string())
        },
        false => download(qualities[0].url.to_string(), filename, qualities[0].audio.to_string()),
    }
}
