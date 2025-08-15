use rand::Rng;
use std::cmp::Ordering;
use std::fs;

use crate::colorization::Color;
use crate::utilities;

pub fn start_rock_paper_scissors() {
    let mut user_score = 0;
    let mut computer_score = 0;

    utilities::clear_console();
    while user_score != 4 && computer_score != 4 {
        let mut rng = rand::rng();
        let computer_choice = match rng.random_range(0..=2) {
            0 => "rock".to_string(),
            1 => "paper".to_string(),
            2 => "scissors".to_string(),
            _ => unreachable!(),
        };

        println!("[Computer] Alright! Rock, paper, scissors, go!:");
        let user_choice = utilities::input("You play: ");
        if user_choice == "stop" {
            return;
        } else if user_choice != "rock" && user_choice != "paper" && user_choice != "scissors" {
            println!(
                "[Computer] I couldn't understand what you typed. If you wish to stop playing, type 'stop'"
            );
            continue;
        }
        if user_choice == computer_choice {
            GameResult::Draw.print_result(&computer_choice);
        } else {
            match (computer_choice.as_str(), user_choice.as_str()) {
                ("rock", "paper") => {
                    GameResult::Loss.print_result(&computer_choice);
                    computer_score += 1;
                }
                ("scissors", "paper") => {
                    GameResult::Loss.print_result(&computer_choice);
                    computer_score += 1;
                }
                ("paper", "rock") => {
                    GameResult::Loss.print_result(&computer_choice);
                    computer_score += 1;
                }
                _ => {
                    GameResult::Win.print_result(&computer_choice);
                    user_score += 1;
                }
            }
        }
        utilities::pause_for(1);
        println!("[Computer] The current score is: Me {computer_score} - You {user_score}");
        utilities::pause_for(1);
    }

    let replay_choice = utilities::input("Want to play again? y/n: ");
    if replay_choice == "y" {
        start_rock_paper_scissors();
    }
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn print_result(&self, computer_plays: &str) {
        match self {
            GameResult::Win => println!(
                "{} {}",
                "[Computer] You win! I played".green(),
                computer_plays.green()
            ),
            GameResult::Loss => println!(
                "{} {}",
                "[Computer] Haha you lose! I played".red(),
                computer_plays.red()
            ),
            GameResult::Draw => println!("[Computer] It's a draw! I played {computer_plays} too"),
        }
    }
}

pub fn start_guess_the_number() {
    utilities::clear_console();
    println!("{}", "Guess The Number".blue());

    let difficulties = ["Hard - 1 try", "Normal - 3 tries", "Easy - 6 tries"];
    for (i, difficulty) in difficulties.iter().enumerate() {
        println!("[{}] {difficulty}", i + 1);
    }

    let mut tries_left = 3;

    match utilities::input("Select your difficulty: ").as_str() {
        "1" => tries_left = 1,
        "2" => tries_left = 3,
        "3" => tries_left = 6,
        _ => {
            println!("Difficulty not found! Difficulty set to Normal");
        }
    }

    println!("Generating magical number...");
    utilities::pause_for(1);
    println!("Done!");
    let mut rng = rand::rng();

    let random_number: u8 = rng.random_range(0..=100);

    while tries_left > 0 {
        let user_guess = utilities::input("Your guess: ");

        if user_guess == "exit" {
            println!("Game exitted. The number was {random_number}");
            break;
        }

        let user_guess: u8 = match user_guess.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        match user_guess.cmp(&random_number) {
            Ordering::Less => {
                tries_left -= 1;
                if tries_left == 0 {
                    break;
                };

                println!(
                    "The number is higher! You have {tries_left} {} left.",
                    if tries_left > 1 { "tries" } else { "try" }
                );
            }
            Ordering::Greater => {
                tries_left -= 1;
                if tries_left == 0 {
                    break;
                };

                println!(
                    "The number is lower! You have {tries_left} {} left.",
                    if tries_left > 1 { "tries" } else { "try" }
                );
            }
            Ordering::Equal => {
                println!("You guessed it! The number was {random_number}");
                match utilities::input("Game finished. Want to play again? [y/n]: ").as_str() {
                    "y" => start_guess_the_number(),
                    "n" => return,
                    _ => return,
                }
                return;
            }
        }
    }

    if tries_left == 0 {
        println!("You lost! The number was {random_number}");
        match utilities::input("Game over. Want to play again? [y/n]: ").as_str() {
            "y" => start_guess_the_number(),
            "n" => return,
            _ => return,
        }
    }
}

pub fn start_hangman() {
    let category = get_category();
    let word = get_word(&category);
    utilities::clear_console();
    println!("{}", "Hangman".blue());

    println!("Category: {category} Word: {word}");
    println!("Game's not finished");
}

fn get_category() -> String {
    let word_files: Vec<_> = match fs::read_dir("./words") {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading notes: {e}");
            return "ErrorXD".to_string();
        }
    }
    .collect(); // Bunch of Result<DirEntries>

    let mut rng = rand::rng();
    let random_number = rng.random_range(0..word_files.len()); // Remember that indexes are
    // exclusive :)

    let file_path = match &word_files[random_number] {
        Ok(f) => f,
        Err(e) => {
            println!("An error occured: {e}");
            return "ErrorXD".to_string(); // I think this won't be reached, so I just put a silly
            // output for simplicity :p
        }
    };
    file_path
        .file_name()
        .into_string()
        .unwrap()
        .replace(".txt", "")
}
fn get_word(category: &str) -> String {
    let words: String = fs::read_to_string(format!("./words/{}.txt", category)).unwrap();

    let split_words: Vec<&str> = words.split('\n').collect();

    let mut rng = rand::rng();
    let random_number = rng.random_range(0..split_words.len() - 1); // -1 because there's always a
    // final extra line for some reason.

    let final_word = split_words[random_number];
    final_word.to_string()
}
