use colorism::{foreground::Fore, util::RESET};
use crate::utils::download::download;
use crate::utils::choose::choose;
use crate::modules::twitter;
use crate::logger;

pub fn get(url: String, filename: String) {
    let qualities = twitter::get_video(url);

    logger::success(&format!("{} qualities available...\n", qualities.len()));

    let qualities_borrow = &qualities;
    for (k, format) in qualities_borrow.into_iter().enumerate() {
        println!("{}[{k}]{} {}", Fore::color(Fore::BdBlue), RESET, format.quality);
    
    }

    download(qualities[choose()].url.to_string(), filename);
}
