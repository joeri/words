extern crate words;

use words::wordsearch::{Frequency, WordMatcher};

fn main() {
    let alphabet = vec!(Frequency { item: 'a', count: 2 }, Frequency { item: 'b', count: 3 }, Frequency { item: 'c', count: 1 });
    let wm = WordMatcher { alphabet: alphabet, length: Some(4) };

    println!("{}", words::wordsearch::matches(&wm, "acbba"));
}
