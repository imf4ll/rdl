use dialoguer::{theme::ColorfulTheme, Select};
use crate::utils::types::Format;

pub fn choose(qualities: &Vec<Format>) -> usize {
    let qualities_vec = qualities[..]
        .iter()
        .map(|i| i.quality.clone())
        .collect::<Vec<String>>();

    Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&qualities_vec)
        .interact()
        .unwrap()
}
