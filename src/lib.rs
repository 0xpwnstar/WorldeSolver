pub mod algorithms;

use std::{collections::HashSet, sync::Arc};

const DICTIONARY: &str = include_str!("../dictionary.txt");


pub struct Wordle {
    dictionary: HashSet<&'static str>
}

impl Wordle  {
    pub fn new() -> Self {
        Self { dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
            line.split_once(' ').expect("must be a word").0
        }))
        }
    }

///play 6 rounds where it invokes Guesser
    pub fn play<G : Guesser>(&self, answer: &'static str,mut guessser: G ) -> Option<usize>{
        let mut history = Vec::new();
        //wordle only allows six guesses
        for i in 1..=32 {
            let guess = guessser.guess(&history);
            if guess == answer {
                return Some(i)
            }
            assert!(self.dictionary.contains(&*guess));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess{ 
                word: guess,
                mask: correctness
            });
        }
    None
    }
}

impl Correctness {
    fn  compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(),5);
        let mut c = [Correctness::Wrong; 5];
        for (i,(a,g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }
        let mut used = [false;5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        for (i,g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }
            if answer.chars().enumerate().any(|(i,a)| {
                if a==g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Wrong, Self::Misplaced],
            [Self::Correct, Self::Wrong, Self::Misplaced],
            [Self::Correct, Self::Wrong, Self::Misplaced],
            [Self::Correct, Self::Wrong, Self::Misplaced],
            [Self::Correct, Self::Wrong, Self::Misplaced]      
        )
        .map(|(a,b,c,d,e)| [a, b, c, d, e]) 
    
    }

}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum Correctness {
    /// Green
    Correct,
    ///yellow
    Misplaced,
    ///Gray
    Wrong,
}

pub struct Guess {
    word : String,
    mask: [Correctness; 5]
}

impl Guess {
    pub fn matches(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), word.len());
        let mut used = [false; 5];
        for (i,((g,&m),w)) in self
        .word
        .chars().zip(&self.mask)
        .zip(word.chars())
        .enumerate() 
        {
            
            
            if m == Correctness::Correct {
                if g != w {
                    return false
                } else {
                    used[i] = true;
                }
            }
        }
        for (i,((g,&m),w)) in self.word.chars().zip(&self.mask).zip(word.chars()).enumerate() {
            
            if m == Correctness::Correct {
                continue;
            }
            
            let mut plausible = true;

            if self.word.chars().zip(&self.mask).enumerate().any(|(j,(g,m))| {
                if g != w {
                    return false;
                }
                if used[j]{
                    return false;
                }
                match m {
                    Correctness::Correct => unreachable!("all "),
                    Correctness::Misplaced if j==i => {
                        plausible = false;
                        return false;
                    }

                    Correctness::Misplaced => {
                        used[j] = true;
                        return true;
                    }
                    Correctness::Wrong => {

                        plausible = false;
                        return false;
                    }
                }

            }) && plausible{
                assert!(plausible)
            } else if !plausible{
                return false;
            }else {

            }
        }

        true
    }
}

pub trait Guesser {
    fn guess(&mut self,history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String{
    fn guess(&mut self,history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history: ident| $impl: block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self,$history: &[Guess]) -> String{
                $impl
            }
        }
        G
    }};
}



#[cfg(test)]
mod tests {
    macro_rules!  mask {
        (M) => {$crate::Correctness::Misplaced};
        (C) => {$crate::Correctness::Correct};
        (W) => {$crate::Correctness::Wrong};
        ($($c:tt)+) => {[
            $(mask!($c)),+
        ]}
    }
    

    mod guess_matcher {
        use crate::Guess;

        #[test]
        fn matches() {
            assert!(Guess {
                word: "baaaa".to_string(),
                mask: mask![W C M W W]
            }.matches("aaccc"))

        }
    }

    mod game {
        use crate::{Guess, Guesser, Wordle};

        #[test]
        fn play() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                "moved".to_string()
            });
            assert_eq!(w.play("moved",guesser),Some(1));
        }


    }
    mod compute {
        use  crate::Correctness;



        macro_rules!  mask {
            (M) => {Correctness::Misplaced};
            (C) => {Correctness::Correct};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }
        #[test]
        fn basic( ) {
            assert_eq!(
                Correctness::compute("abcde", "abcde"),
                mask![C C C C C]
            );
        }
        #[test]
        fn yellow( ) {
            assert_eq!(
                Correctness::compute("abcde", "eabcd"),
                [Correctness::Misplaced; 5]
            );
        }
        #[test]
        fn gray( ) {
            assert_eq!(
                Correctness::compute("abcde", "fghij"),
                [Correctness::Wrong; 5]
            );
        }

        #[test]
        fn repeat_gray( ) {
            assert_eq!(
                Correctness::compute("aacde", "acaoa"),
                mask![C M M W W]
            );
        }
    }
}