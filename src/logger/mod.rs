use std::process::exit;

use colorism::{foreground::Fore, util::RESET};

pub fn success(message: &str) {
    println!("{}√ {message}{RESET}", Fore::color(Fore::BdGreen));
}

pub fn warn(message: &str) {
    println!("{}⚠️ {message}{RESET}\n", Fore::color(Fore::BdYellow));
}

pub fn error(message: &str) {
    println!("{}✕ {message}{RESET}", Fore::color(Fore::BdRed));

    exit(3);
}
