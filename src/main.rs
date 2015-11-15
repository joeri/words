extern crate words;

use words::wordsearch::{Frequency, WordMatcher};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let alphabet = vec!(Frequency { item: 'a', count: 2 }, Frequency { item: 'b', count: 3 }, Frequency { item: 'c', count: 1 });
    let wm = WordMatcher { alphabet: alphabet, length: Some(4) };

    let res = File::open("/usr/share/dict/words");
    match res {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let l = line.unwrap();
                if wm.matches(&l) {
                    println!("{}", l);
                }
            }
        },
        Err(_) => println!("Could not open dictionary file"),
    }
}
