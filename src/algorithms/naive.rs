use crate::{ Guess, Guesser};

pub struct Naive;

impl Guesser for Naive {
    fn guess(&mut self,history: &[Guess]) -> String {
        todo!();
    }
}

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}