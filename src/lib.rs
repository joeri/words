pub mod wordsearch {

    pub enum LengthConstraint {
        Exact(usize),
        Max(usize),
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
            let found : &mut Vec<Frequency> = &mut starting_frequencies(&self);

            match self.length {
                Exact(length) => if length != string.chars().count() { return false },
                Max(length)   => if length  < string.chars().count() { return false },
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
