use help::{start, cur_word, input, remove_duplicates, end};
fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let (games, words, guesses) = start(); 

    let mut word;
    let mut guessed;
    let mut letters;
    let mut r_guessed_letters;
    let mut w_guessed_letters;
    let mut tries;
    let mut ask;
    let mut used;
    let mut chances;
    let mut guess;

    for g in 0..games {

        println!("Game {}!", g+1);
        word = words.get(g).unwrap();
        guessed = false;  
        letters = remove_duplicates(word.split("").collect::<Vec<&str>>());
        r_guessed_letters = vec!["".to_owned()];
        w_guessed_letters = vec![];
        tries = 0;
        ask = format!("You are now guessing: {}", "_".repeat(word.len()));
        used = "You have guessed these letter(s):\n".to_owned();
        chances = format!("You have {} more chance(s).\nGuess a letter or the word:", guesses-w_guessed_letters.len());

        while w_guessed_letters.len() < guesses {
            print!("{}\n{}{} ", ask, used, chances);
            guess = input().trim().to_owned();
            if guess == "EXIT" {
                end();
                break;
            }
            if guess == *word {
                guessed = true;
                break;  
            }
            if (guess.len() != 1) | !("abcdefghijklmnopqrstuvwxyz").contains(&guess) {
                println!("{} isn't a letter.\n", guess);
                continue;
            }
            if w_guessed_letters.contains(&guess) | r_guessed_letters.contains(&guess) {
                println!("You already guessed {}!\n", guess);
                continue;
            }
            tries += 1;
            if used == *"You have guessed these letter(s):\n" {
                used = format!("{}{}", used, guess);
            } else {
                used = format!("{}{}{}", used, ", ", guess);
            }
            if letters.contains(&guess.as_str()) {
                println!("{} was a letter!", guess);
                r_guessed_letters.push(guess);
                ask = cur_word(word, &r_guessed_letters);
                println!("{}", letters[0]);
                if r_guessed_letters.len() == letters.len() {
                    guessed = true;
                    break;
                }
            } else {
                println!("Nope, {} isn't in the word!\n", guess);
                w_guessed_letters.push(guess);
            }
            chances = format!("\nYou have {} more chance(s).\nGuess a letter or the word:", guesses-w_guessed_letters.len());
        }
        if guessed {
            println!("You did it in {} tries!\nThe word was {}.\n\n", tries, word);
        } else {
            println!("Better luck next time!\nThe word was {}.\n", word);
        }
    }
    end();
}