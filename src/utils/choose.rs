use std::io::stdin;

pub fn choose() -> usize {
    let mut choose_str = String::new();

    stdin()
        .read_line(&mut choose_str)
        .expect("Failed to parse choose");

    let choose: usize = choose_str
        .trim()
        .parse()
        .expect("Failed to parse choose to number");

    choose
}
