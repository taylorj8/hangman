use std::io::{self, Write};

mod strings;

fn main() {

    let mut record = (0, 0);    // record wins and losses
    let mut playing = true;
    while playing {
        println!("Welcome to Hangman!\n{}", strings::get_gallows(6));

        // initialise variables needed for the game
        let word = strings::get_random_word();
        let mut blanks = init_blanks(word.len());
        let mut guessed_letters = Vec::with_capacity(26);
        let mut lives = 6;
        let mut correct_guesses = 0;
        let mut in_game = true;

        while in_game {
            // ask for guess
            print!("\n{}\nEnter your guess: ", blanks.clone().into_iter().collect::<String>());
            io::stdout().flush().unwrap();

            // get guess from stdin
            let mut guess_str = String::new();
            io::stdin()
                .read_line(&mut guess_str)
                .expect("Failed to read line");
            let guess_str = guess_str.trim();

            // check guess is valid - single alphabetic character that hasn't already been guessed
            let valid_guess;
            let guess;
            let guess_op = guess_str.chars().next();
            match guess_op {
                Some(g) => {
                    guess = g.to_ascii_lowercase();
                    valid_guess = if guess_str.len() == 1 && guess.is_alphabetic() {
                        if !guessed_letters.contains(&guess) {
                            true
                        } else {
                            println!("{guess} has already been guessed.\n");
                            false
                        }
                    } else {
                        println!("Invalid guess. Only single, alphabetic letters accepted.\n");
                        false
                    };
                },
                None => {
                    valid_guess = false;
                    guess = ' ';
                },
            }

            if valid_guess {
                guessed_letters.push(guess);    // add the letter to the vec of guessed letters

                // store the index of each time the guess occurs in the word
                let mut occurrences = Vec::with_capacity(word.len());
                for (i, letter) in word.chars().enumerate() {
                    if guess == letter {
                        occurrences.push(i);
                    }
                }

                // fill the occurrences into the blanks that are printed, saving the number of correct guesses
                // else if no occurrences, subtract a life, print life lost message
                if occurrences.len() != 0 {
                    for i in occurrences.into_iter() {
                        blanks[i * 2] = guess;
                        correct_guesses = correct_guesses + 1;
                    }
                } else {
                    lives = lives - 1;

                    println!("{guess} is not in the word.");
                    match lives {
                        0 => (),
                        1 => println!("You have 1 life left."),
                        _ => println!("You have {lives} lives left."),
                    }
                }

                // if number of correct guesses is equal to the length of the word, end the game and print vectory message
                // else if out of lives, end the game and print game over message
                // else just print the gallows
                if correct_guesses == word.len() {
                    in_game = false;
                    record.0 = record.0 + 1;
                    println!("\n{}\nYou win!\n{}", blanks.clone().into_iter().collect::<String>(), strings::get_gallows(-1));
                } else if lives == 0 {
                    in_game = false;
                    record.1 = record.1 + 1;
                    println!("You lost :( The word was {word}.\n{}", strings::get_gallows(0))
                } else {
                    println!("{}", strings::get_gallows(lives));
                }
            }
        }
        playing = ask_to_repeat();
        println!("\n");
    }
    println!("Thanks for playing! You won {} and lost {}.", record.0, record.1);
}


// asks the user if they want to play again and handles incorrect input
fn ask_to_repeat() -> bool {
    print!("Do you want to play again? (y/n): ");
    io::stdout().flush().unwrap();
    
    loop {
        let mut input = String::new();
        io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        let input = input.trim();
        match input {
            "y" => return true,
            "n" => return false,
            _ => {
                print!("Invalid input - Enter y or n: ");
                io::stdout().flush().unwrap();
            }
        }
    };
}


// initialises the blanks that are printed to the screen
fn init_blanks(word_length: usize) -> Vec<char> {
    let mut blanks = Vec::with_capacity(word_length*2-1);
    for _ in 0..word_length - 1 {
        blanks.push('_');
        blanks.push(' ');
    }
    blanks.push('_');

    blanks
}
