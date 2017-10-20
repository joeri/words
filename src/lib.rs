pub mod wordsearch {

    pub enum LengthConstraint {
        Exact(usize),
        Min(usize),
        Max(usize),
        Between(usize, usize),
        Noconstraint,
    }

    #[derive(Clone,Copy)]
    pub struct Frequency {
        pub item: char,
        pub count: i8,
    }

    pub struct WordMatcher {
        pub alphabet: Vec<Frequency>,
        pub length: LengthConstraint,
    }

    fn starting_frequencies(wm: &WordMatcher) -> Vec<Frequency> {
        wm.alphabet.iter().map(|f|
            Frequency { count: 0, item: f.item }
        ).collect()
    }

    use self::LengthConstraint::*;

    impl WordMatcher {

        pub fn matches(&self, string: &str) -> bool {
            if !self.matches_length_constraint(string) {
                return false;
            }
            self.matches_frequencies(string)
        }

        pub fn matches_frequencies(&self, string: &str) -> bool {
            let found : &mut Vec<Frequency> = &mut starting_frequencies(&self);

            for c in string.chars() {
                let position = self.alphabet.iter().position(|f| c == f.item);

                match position {
                    Some(i) => {
                        found[i].count += 1;
                        if found[i].count > self.alphabet[i].count {
                            return false
                        }
                    },
                    None    => return false
                }
            }

            true
        }

        pub fn matches_length_constraint(&self, string: &str) -> bool {
            match self.length {
                Exact(length)     => length == string.chars().count(),
                Min(length)       => length <= string.chars().count(),
                Max(length)       => length >= string.chars().count(),
                Between(min, max) => {
                    let count = string.chars().count();
                    count < min || max <= count
                },
                _ => true,
            }
        }


        pub fn from_alphabet(alphabet: &str, length: LengthConstraint) -> WordMatcher {
            let frequencies : &mut Vec<Frequency> = &mut Vec::with_capacity(alphabet.chars().count());

            for c in alphabet.chars() {
                let position = frequencies.iter().position(|f| c == f.item);

                match position {
                    Some(pos) => {
                        let freq = frequencies.swap_remove(pos);
                        frequencies.push(Frequency { item: c, count: freq.count + 1 });
                    },
                    None => frequencies.push(Frequency { item: c, count: 1 }),

                };
            }

            WordMatcher { alphabet: frequencies.clone(), length: length }
        }

    }

}
