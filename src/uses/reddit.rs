use crate::utils::download::download;
use crate::utils::choose::choose;
use crate::modules::reddit;
use crate::logger;

pub fn get(url: String, filename: String) {
    let qualities = reddit::get_video(url);

    logger::success(&format!("{} qualities available...\n", qualities.len()));

    let choose = &qualities[choose(&qualities)];

    match qualities.len() > 1 {
        true => download(choose.url.to_string(), filename, choose.audio.to_string()),
        false => download(qualities[0].url.to_string(), filename, choose.audio.to_string()),
    }
}
