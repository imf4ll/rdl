use crate::utils::download::download;
use crate::utils::choose::choose;
use crate::modules::facebook;
use crate::logger;

pub fn get(url: String, filename: String) {
    let qualities = facebook::get_video(url);

    logger::success(&format!("{} qualities available...\n", qualities.len()));

    download(qualities[choose(
        qualities[..]
            .iter()
            .map(|i| i.quality.clone())
            .collect::<Vec<String>>()
    )].url.to_string(), filename);
}
