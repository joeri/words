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

    pub fn matches(wm: WordMatcher, string: &str) -> bool {
        let found : &mut Vec<Frequency> = &mut wm.alphabet.clone().into_iter().map(|f|
            Frequency { count: 0, item: f.item }
        ).collect();

        match wm.length {
            Some(length) => if length < string.chars().count() { return false },
            _ => (),
        }

        for c in string.chars() {
            let position = wm.alphabet.clone().into_iter().position(|f| c == f.item);

            match position {
                Some(i) => {
                    found[i].count = found[i].count + 1;
                    if found[i].count > wm.alphabet[i].count {
                        return false
                    }
                },
                None    => return false
            }
        }

        true
    }

}
