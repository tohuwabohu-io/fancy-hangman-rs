use std::io::stdin;
use clap::{Parser, Subcommand};
use console::style;

use wordle_cli::lang::locale::replace_unicode;
use wordle_cli::dictionary::{Dictionary, get_dictionary};
use wordle_cli::maintenance::import::do_import;

/// Play wordle, a word guessing game!
#[derive(Parser)]
struct Arguments {
    /// Language of the dictionary that will be loaded. Supported: EN, DE. Defaults to EN.
    #[clap(short, long)]
    language: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Extend the dictionary
    Import {
        #[clap(short, long)]
        /// File to import. Requires entries to be separated by newlines.
        source_file: String,
        /// Language of the dictionary to import.
        #[clap(short, long)]
        import_language: String
    }
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    let lang = match args.language {
        None => String::from("en"),
        Some(flag) => flag
    };

    let lang = lang.as_str();

    match args.command {
        Some(command) => {
            match command {
                Commands::Import { source_file, import_language } => {
                    do_import(source_file, import_language.as_str())?;
                }
            }
        }
        _ => {
            start_game(lang);
        }
    }

    Ok(())
}

fn start_game(lang: &str) {
    print_welcome();

    let dictionary = get_dictionary(lang);
    let solution_option = dictionary.get_random_word();

    match solution_option {
        None => println!("Maybe the dictionary is empty?"),
        Some(solution) => {

            if solution.guessed {
                check_word(&solution.word, &solution.word);

                println!("You won! Come back tomorrow!");
            } else {
                let max_attempts = 6;

                let mut full_match: bool = false;

                let mut counter = 0;
                while counter < max_attempts {
                    let attempt: String = read_input(5, lang);

                    match dictionary.find_word(&attempt) {
                        Some(_) => {
                            let guesses: i32 = max_attempts - counter - 1;
                            full_match = check_word(&solution.word, &attempt);

                            if full_match == true {
                                break;
                            } else {
                                if guesses > 1 {
                                    println!("You now have {} guesses.", guesses);
                                } else {
                                    println!("This is your last guess.");
                                }
                            }

                            if guesses == 0 { println!("Better luck next time!") }

                            counter += 1;
                        },
                        None => println!("The guessed word is not in the word list.")
                    }
                }

                if full_match {
                    println!("Congratulations! You won!");
                    dictionary.guessed_word(solution);
                }
            }
        }
    }
}

fn read_input(word_len: usize, lang: &str) -> String {
    let mut input: String = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();
        let polished = replace_unicode(input.to_lowercase().trim(), lang);

        if !validate_user_input(&polished, word_len) {
            println!("Invalid input: Your guess must have a size of {} characters. You entered {} characters.", word_len, polished.len());

            input.clear();
        } else {
            input = polished.to_lowercase();

            break;
        }
    }

    input
}

fn validate_user_input(user_input: &str, expected_len: usize) -> bool {
    user_input.len() == expected_len
}

fn check_word(solution_word: &str, guessed_word: &str) -> bool {
    let guessed_characters: Vec<char> = guessed_word.chars().collect();
    let solution_characters: Vec<char> = solution_word.chars().collect();

    for i in 0..guessed_word.len() {
        let index: Option<usize> = solution_word.find(guessed_characters[i]);

        match index {
            Some(_index) => {
                if solution_characters[i] == guessed_characters[i] {
                    print!("{} ", style(guessed_characters[i].to_string()).green())
                } else {
                    print!("{} ", style(guessed_characters[i].to_string()).yellow())
                }
            }
            None => { print!("{} ", guessed_characters[i]) }
        }
    }

    println!();

    // check for full match
    if String::from(solution_word).eq(guessed_word) {
        return true;
    }

    false
}

fn print_welcome() {
    println!(r#"
____    __    ____  ______   .______       _______   __       _______          ______  __       __
\   \  /  \  /   / /  __  \  |   _  \     |       \ |  |     |   ____|        /      ||  |     |  |
 \   \/    \/   / |  |  |  | |  |_)  |    |  .--.  ||  |     |  |__    ______|  ,----'|  |     |  |
  \            /  |  |  |  | |      /     |  |  |  ||  |     |   __|  |______|  |     |  |     |  |
   \    /\    /   |  `--'  | |  |\  \----.|  '--'  ||  `----.|  |____        |  `----.|  `----.|  |
    \__/  \__/     \______/  | _| `._____||_______/ |_______||_______|        \______||_______||__|

Welcome! Guess today's word in 6 guesses.
_ _ _ _ _
    "#)
}

#[cfg(test)]
#[test]
fn test_validate_user_input() {
    assert!(validate_user_input(
        replace_unicode("schön", "de").as_str(), 6
    ));

    assert!(validate_user_input(
        replace_unicode("schön", "en").as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("lüge", "de").as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("lüge", "en").as_str(), 4
    ));

    assert!(validate_user_input(
        replace_unicode("howdy", "de").as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("howdy", "en").as_str(), 5
    ));

    assert!(validate_user_input(
        replace_unicode("wölfe", "de").as_str(), 6
    ));

    assert!(validate_user_input(
        replace_unicode("wölfe", "en").as_str(), 5
    ));
}