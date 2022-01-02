#![allow(unused)]

pub fn reverse(input: &str) -> String {
    input
        .to_string()
        .chars()
        .rev()
        .collect()
}
