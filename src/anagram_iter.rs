#![allow(unused)]

use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    possible_anagrams
        .iter()
        .cloned()
        .filter(|&possible_anagram| {
            let anagram = possible_anagram.to_lowercase();
            sort(&anagram) == sorted_word && anagram != word
        })
        .collect()
}

fn sort<'a>(word: &'a str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort_unstable();
    sorted_word
}
