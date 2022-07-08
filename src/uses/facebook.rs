use std::io::stdin;

use colorism::{foreground::Fore, util::RESET};
use crate::utils::download::download;
use crate::modules::facebook;
use crate::logger;

pub fn get(url: String, filename: String) {
    let qualities = facebook::get_video(url);

    logger::success(&format!("{} qualities available...\n", qualities.len()));

    let qualities_borrow = &qualities;
    for (k, format) in qualities_borrow.into_iter().enumerate() {
        println!("{}[{k}]{} {} ({})", Fore::color(Fore::BdBlue), RESET, format.quality, format.format);
    
    }

    let mut choose_str = String::new();

    stdin()
        .read_line(&mut choose_str)
        .expect("Failed to parse choose");

    let choose: usize = choose_str
        .trim()
        .parse()
        .expect("Failed to parse choose to number");

    download(qualities[choose].url.to_string(), filename);
}
