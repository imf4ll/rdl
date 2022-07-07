use colorism::{foreground::Fore, util::RESET};
use crate::utils::download::download;
use crate::modules::twitter;
use std::io::stdin;

pub fn get(url: String, filename: String) {
    let qualities = twitter::get_video(url);

    println!("{}[>] {} qualities available...{}\n", Fore::color(Fore::BdGreen), qualities.len(), RESET);

    let qualities_borrow = &qualities;
    for (k, format) in qualities_borrow.into_iter().enumerate() {
        println!("{}[{k}]{} {}", Fore::color(Fore::BdBlue), RESET, format.quality);
    
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
