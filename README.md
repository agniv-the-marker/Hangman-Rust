# Hangman in Rust

A simple game made in Rust. Use cargo run to start. 

## Rules

For the first two questions, input integers greater then 0. For entering a guess, you may do either a letter or the word itself.

## Skills Needed/Learned

Learned basic rust syntax, cargo, 'use', input/output, etc.

## Sample Run

> (*italicized text* is input)

How many games do you want to play? (Please enter a number greater then 0)
*2*

How many wrong guesses do you want? (Please enter a number greater then 0)

*3*

Game 1!
You are now guessing: _______    
You have guessed these letter(s):
You have 3 more chance(s).       
Guess a letter or the word: *e*   
e was a letter!

You are now guessing: _____e_    
You have guessed these letter(s):
e
You have 3 more chance(s).       
Guess a letter or the word: *t*   
t was a letter!

You are now guessing: ____te_    
You have guessed these letter(s):
e, t
You have 3 more chance(s).       
Guess a letter or the word: *a*    
a was a letter!

You are now guessing: a__ate_
You have guessed these letter(s):
e, t, a
You have 3 more chance(s).
Guess a letter or the word: *s*
s was a letter!

You are now guessing: a__ates
You have guessed these letter(s):
e, t, a, s
You have 3 more chance(s).
Guess a letter or the word: *n*
n was a letter!

You did it in 5 tries!
The word was annates.


Game 2!
You are now guessing: ____________
You have guessed these letter(s):
You have 3 more chance(s).
Guess a letter or the word: *e*
e was a letter!

You are now guessing: ____e_____e_
You have guessed these letter(s):
e
You have 3 more chance(s).
Guess a letter or the word: *t*
Nope, t isn't in the word!

You are now guessing: ____e_____e_
You have guessed these letter(s):
e, t
You have 2 more chance(s).
Guess a letter or the word: *a*
Nope, a isn't in the word!

You are now guessing: ____e_____e_
You have guessed these letter(s):
e, t, a
You have 1 more chance(s).
Guess a letter or the word: *z*
Nope, z isn't in the word!

Better luck next time!
The word was huggermugger.

Press enter to exit
