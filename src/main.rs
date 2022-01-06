use rand::Rng;
use std::fs;
use std::io::{self, Read};
fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let word = get_word();
    let mut guessed = false;
    let splitletters = word.split("");
    let letters = remove_duplicates(splitletters.collect::<Vec<&str>>());
    let mut rguessedletters: Vec<String> = vec!["".to_owned()];
    let mut wguessedletters: Vec<String> = vec![];
    let mut tries = 0;
    while wguessedletters.len() < 10 {
        let mut ask = "You are now guessing: ".to_owned();
        let mut cur = "".to_owned();
        for l in word.split("") {
            cur = if !(rguessedletters.contains(&l.to_owned())) {
                [cur, "_".to_owned()].join("")
            } else {
                [cur, l.to_owned()].join("")
            };
        }
        ask = ask + &cur.to_owned();
        let mut guessedletters = "".to_owned();
        for r in &rguessedletters {
            guessedletters = [guessedletters, r.to_owned(), ", ".to_owned()].join("");
        }
        for w in &wguessedletters {
            guessedletters = [guessedletters, w.to_owned(), ", ".to_owned()].join("");
        }
        if guessedletters.len() > 2 {
            guessedletters = guessedletters[1..guessedletters.len()-2].to_owned();
        }
        println!("{}\nYou have guessed these letter(s):\n{}\nYou have {} more chance(s).", ask, guessedletters, 10-wguessedletters.len());
        print!("Guess a letter or the word: ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => { },
            Err(err) => println!("Could not parse input: {}", err)
        }    
        guess = guess.trim().to_owned();
        if guess == word {
            guessed = true;
            break;  
        }
        if !(guess.len() == 1) | !("abcdefghijklmnopqrstuvwxyz").contains(&guess) {
            println!("{} isn't a letter.\n", guess);
            continue;
        }
        if wguessedletters.contains(&guess) | rguessedletters.contains(&guess) {
            println!("You already guessed {}!\n", guess);
            continue;
        }
        tries = tries + 1;
        if letters.contains(&&*guess) {
            println!("{} was a letter!\n", guess);
            rguessedletters.push(guess);
            println!("{}", letters[0]);
            if rguessedletters.len() == letters.len() {
                guessed = true;
                break;
            }
        } else {
            println!("Nope, {} isn't in the word!\n", guess);
            wguessedletters.push(guess);
        }
    }
    if guessed {
        println!("You did it in {} tries!\nThe word was {}.", tries, word);
    } else {
        println!("Better luck next time!\nThe word was {}.", word);
    }
    println!("Press enter to exit");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok();
}


fn get_word() -> String {
    let temp = fs::read_to_string("src/words.txt").expect("Something went wrong reading the file");
    let contents = temp.trim().split("\n");
    let vec = contents.collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    vec[rng.gen_range(0..vec.len())].trim().to_owned()
}

fn remove_duplicates(mut arr : Vec<&str>) -> Vec<&str> {
    arr.sort();
    arr.dedup();
    arr
}

