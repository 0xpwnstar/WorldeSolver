const GAMES : &str = include_str!("../answers.txt");


fn main() {
    let w = roget::Wordle::new();
    for answer in GAMES.split_whitespace() {
        println!("answer is {}",answer);
        let guesser = roget::algorithms::Naive::new();
        if let Some(score) = w.play(answer, guesser){
            println!("guessed {} in {}",answer ,score);
        }
    }
    println!("Hello, world!");
}

