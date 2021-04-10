
pub mod dictionary {

    extern crate rand;

    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    use self::rand::distributions::{IndependentSample, Range};
    use self::rand::ThreadRng;

    pub struct Dictionary {
        nouns: Vec<String>,
        adjectives: Vec<String>,
        rng: ThreadRng,
    }

    impl Dictionary {
        fn load(file_name: String) -> Vec<String> {
            let read_result = File::open(file_name);
            match read_result {
                Ok(f) => {
                    let mut vector: Vec<String> = vec![];
                    let file = BufReader::new(&f);
                    for line in file.lines() {
                        let l = line.unwrap();
                        vector.push(l);
                    }
                    vector
                }
                Err(_) => vec![],
            }
        }

        pub fn new() -> Dictionary {
            Dictionary {
                nouns: Dictionary::load("n.txt".to_string()),
                adjectives: Dictionary::load("a.txt".to_string()),
                rng: rand::thread_rng(),
            }
        }

        pub fn word(&mut self) -> String {
            let adjectives_range = Range::new(0, self.adjectives.len() - 1);
            let nouns_range = Range::new(0, self.nouns.len() - 1);

            let adjectives_index = adjectives_range.ind_sample(&mut self.rng);
            let nouns_index = nouns_range.ind_sample(&mut self.rng);

            let x = &self.adjectives[adjectives_index];
            let y = &self.nouns[nouns_index];

            (x.to_owned() + "_" + &y)
        }
    }
}
