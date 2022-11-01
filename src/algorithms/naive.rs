use crate::{ Guess, Guesser, DICTIONARY, Correctness};
use std::{collections::HashMap};

pub struct Naive{
    remaining: HashMap<&'static str, usize>,
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64
}

impl Guesser for Naive {
    fn guess(&mut self,history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word,_| last.matches(word));
        }
        if history.is_empty() {
            return "tares".to_string();
        }
        let total_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();
        let mut best :Option<Candidate> = None;
        for (&word, _) in &self.remaining {
            let mut sum = 0.0;
            for pattern in Correctness::patterns() {
                let mut patttern_total = 0;

                
                for (candidate,count) in &self.remaining {
                    let g = Guess {
                        word: word.to_string(),
                        mask: pattern,
                    };
                    if g.matches(candidate){
                        patttern_total += count;
                    }
                }
                if patttern_total == 0 {
                    continue;
                }
                let p_of_this_pattern = patttern_total as f64 / total_count as f64;
                sum += p_of_this_pattern*p_of_this_pattern.log2();
            }
            let goodness = 0.0 - sum;
            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate { word,count: 0, goodness})
                }

            } else {
                best = Some(Candidate{
                    word,
                    count: 0,
                    goodness
                });
            }
        }
        eprint!("guess is {:?}",best.unwrap().word);
        best.unwrap().word.to_string()
        
    }
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining : HashMap::from_iter(DICTIONARY.lines().map(|line|  {
                let (word, count) = line.split_once(' ')
                .expect("expect a word and a number");
                let count = count.parse().expect("num please");
                (word, count)
            }))
        }
    }
}