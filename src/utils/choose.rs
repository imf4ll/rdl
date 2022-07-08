use dialoguer::{theme::ColorfulTheme, Select};

pub fn choose(qualities: Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&qualities)
        .interact()
        .unwrap()
}
