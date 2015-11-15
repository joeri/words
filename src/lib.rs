pub mod wordsearch {

    #[derive(Clone,Copy)]
    pub struct Frequency {
        pub item: char,
        pub count: i8,
    }

    pub struct WordMatcher {
        pub alphabet: Vec<Frequency>,
        pub length: Option<usize>,
    }

    fn starting_frequencies(wm: &WordMatcher) -> Vec<Frequency> {
        wm.alphabet.iter().map(|f|
            Frequency { count: 0, item: f.item }
        ).collect()
    }

    impl WordMatcher {

        pub fn matches(&self, string: &str) -> bool {
            let found : &mut Vec<Frequency> = &mut starting_frequencies(&self);

            match self.length {
                Some(length) => if length < string.chars().count() { return false },
                _ => (),
            }

            for c in string.chars() {
                let position = self.alphabet.iter().position(|f| c == f.item);

                match position {
                    Some(i) => {
                        found[i].count = found[i].count + 1;
                        if found[i].count > self.alphabet[i].count {
                            return false
                        }
                    },
                    None    => return false
                }
            }

            true
        }

    }

}
