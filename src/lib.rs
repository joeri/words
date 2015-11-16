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

    }

}
