#![feature(collections)]

fn sort(word: &String) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort();
    let mut output = String::with_capacity(sorted.len());
    for c in sorted.drain() {
        output.push(c);
    }
    output
}

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    // ...
}
