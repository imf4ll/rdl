use crate::modules::twitter;
use colorism::{foreground::Fore, util::RESET};
use std::io::stdin;
use crate::utils::download::download;

pub fn get(url: String) {
    let formats = twitter::get_video(url);

    let qualities_borrow = &formats;

    println!("{}[>] {} qualities available...{}\n", Fore::color(Fore::BdGreen), qualities_borrow.len(), RESET);

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

    download(formats[choose].url.to_string());
}
