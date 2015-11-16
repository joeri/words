extern crate words;

use words::wordsearch::WordMatcher;
use words::wordsearch::LengthConstraint::*;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::env;
use std::str::FromStr;


fn main() {
    let mut args = env::args();

    let wm = if env::args().count() == 3 {
        let _ = args.next();
        let alphabet = args.next().unwrap();
        let size = usize::from_str(&args.next().unwrap()).unwrap();

        WordMatcher::from_alphabet(&alphabet, Exact(size))
    } else {
        let _ = args.next();
        let alphabet = args.next().unwrap();

        WordMatcher::from_alphabet(&alphabet, Noconstraint)
    };

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
