use rand::Rng;
use std::{io::{self, Read}, fs};

pub fn start() -> (usize, Vec<String>, usize) {
    println!("How many games do you want to play? (Please enter a number greater then 0)");
    let games = input().parse().unwrap_or(0);
    println!("How many wrong guesses do you want? (Please enter a number greater then 0)");
    let guesses = input().parse().unwrap_or(0);
    (games, get_words(games), guesses)
}

pub fn input() -> String {
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut rtn = String::new();
    match io::stdin().read_line(&mut rtn) {
        Ok(_) => { },
        Err(err) => println!("Could not parse input: {}", err)
    }
    rtn.trim().to_owned()
}

pub fn cur_word(word: &str, r_guessed_letters: &[String]) -> String {
    let mut ask = "You are now guessing: ".to_owned();
    let mut cur = "".to_owned();
    for l in word.split("") {
        cur = if !(r_guessed_letters.contains(&l.to_owned())) {
            [cur, "_".to_owned()].join("")
        } else {
            [cur, l.to_owned()].join("")
        };
    }
    ask = ask + &cur;
    ask
}

pub fn get_words(words: usize) -> Vec<String> {
    let temp = fs::read_to_string("src/words.txt").expect("Something went wrong reading the file");
    let contents = temp.trim().split('\n');
    let mut vec = contents.collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    let mut w = vec![];
    for _n in 0..words {
        let i = rng.gen_range(0..vec.len());
        let word = vec[i].trim().to_owned();
        vec.remove(i);
        w.push(word);
        
    }
    w
}

pub fn remove_duplicates(mut arr : Vec<&str>) -> Vec<&str> {
    arr.sort_unstable();
    arr.dedup();
    arr
}

pub fn end() {
    println!("Press enter to exit");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok();
}