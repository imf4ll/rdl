use crate::utils::download::download;
use crate::utils::choose::choose;
use crate::modules::facebook;
use crate::logger;

pub fn get(url: String, filename: String) {
    let qualities = facebook::get_video(url);

    logger::success(&format!("{} qualities available...\n", qualities.len()));

    match qualities.len() > 1 {
        true => download(qualities[choose(&qualities)].url.to_string(), filename, String::from("")),
        false => download(qualities[0].url.to_string(), filename, String::from("")),
    }
}
