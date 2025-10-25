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

    println!("Category: {category}");

    let mut progress = vec!["_".to_string(); word.len()];
    println!("{} ({})", progress.join(" "), progress.len());

    let mut lives_left = 6;
    let mut letter_history: Vec<String> = Vec::new();

    while progress.join("") != word && lives_left > 0 {
        let guess = utilities::input("Your guess: ");
        let message;

        if guess.len() > 1 {
            message = "Type only 1 letter".to_string();
        } else if letter_history.contains(&guess) {
            message = "Letter already guessed".to_string();
        } else if word.contains(&guess) {
            message = "Good job!".to_string();
            progress = fill_progress(&guess, progress, &word);
            letter_history.push(guess);
        } else {
            lives_left -= 1;
            if lives_left == 0 {
                break;
            }
            message = "Wrong guess!".to_string();
            letter_history.push(guess);
        }

        // Reset
        utilities::clear_console();
        println!("{}", "Hangman".blue());
        println!("Category: {category}");
        println!("Letters guessed: {}", letter_history.join(" "));
        println!("{} ({})", progress.join(" "), progress.len());
        println!("{message}");
        println!("Lives left: {lives_left}");
    }
    if progress.join("") == word {
        println!("You gussed the word! Good job");
    } else {
        println!("You didn't guess the word! It was: {word}");
    }
    let replay = utilities::input("Game over. Want to play again? y/n: ");
    if replay == "y" {
        start_hangman()
    }
}

pub fn start_tic_tac_toe() {
    let mut game_squares = vec![[" "; 3]; 3];
    let mut current_player = 1;
    let mut supporting_msg = "";
    let mut winner_text = "";
    let mut title = String::from("Tic Tac Toe - To play, select a square like: 0, 0");

    loop {
        utilities::clear_console();
        println!("{}", title.purple());

        display_tic_tac_toe(&game_squares);

        if !supporting_msg.is_empty() {
            println!("{supporting_msg}");
        }

        if !winner_text.is_empty() {
            println!("{}", winner_text.yellow());
            let play_again = utilities::input("Play again? y/n: ");

            if play_again == "y" {
                start_tic_tac_toe();
            } else {
                break;
            }
        }

        let player_coord_choice = utilities::input(&format!("Player {current_player} to play: "));

        let disect_result = match disect_player_choice(&player_coord_choice, &game_squares) {
            Ok(var) => {
                supporting_msg = "";
                var
            }
            Err(err) => {
                supporting_msg = err;
                utilities::clear_console();
                continue;
            }
        };

        if current_player == 1 {
            game_squares[disect_result.0][disect_result.1] = "X";

            current_player = 2
        } else {
            game_squares[disect_result.0][disect_result.1] = "O";

            current_player = 1
        }

        title = format!("Tic Tac Toe - Player {current_player} to play");

        if check_for_victory(&game_squares, "X") {
            winner_text = "Player 1 won!";
        } else if check_for_victory(&game_squares, "O") {
            winner_text = "Player 2 won!";
        }
    }
}

fn check_for_victory(map: &Vec<[&str; 3]>, symbol: &str) -> bool {
    let mut symbol_count_x = 0;
    let mut symbol_count_y = 0;
    let mut symbol_count_cross_l = 0;
    let mut symbol_count_cross_r = 0;

    for i in 0..3 {
        for j in 0..3 {
            if map[j][i] == symbol {
                symbol_count_x += 1;
            }
            if map[i][j] == symbol {
                symbol_count_y += 1;
            }
        }

        if map[i][i] == symbol {
            symbol_count_cross_l += 1;
        }

        if map[2 - i][2 - i] == symbol {
            symbol_count_cross_r += 1;
        }

        if symbol_count_cross_r == 3
            || symbol_count_cross_l == 3
            || symbol_count_x == 3
            || symbol_count_y == 3
        {
            return true;
        }

        (symbol_count_x, symbol_count_y) = (0, 0);
    }

    false
}

fn disect_player_choice(input: &str, map: &Vec<[&str; 3]>) -> Result<(usize, usize), &'static str> {
    let separated_input: Vec<&str> = input.split([',']).map(|l| l.trim()).collect();
    // println!("{separated_input:?}"); for debugging

    if separated_input.len() != 2 {
        return Err("Incorrect input");
    }

    let x_coord_result = separated_input[0].parse::<usize>();

    let y_coord_result = separated_input[1].parse::<usize>();

    if x_coord_result.is_err() {
        return Err("Invalid input");
    } else if y_coord_result.is_err() {
        return Err("Invalid input");
    } else {
        let x_coord_value = x_coord_result.unwrap();
        let y_coord_value = y_coord_result.unwrap();

        if x_coord_value > 2 || y_coord_value > 2 {
            return Err("Valid squares: 0, 1 or 2");
        } else if map[x_coord_value][y_coord_value] != " " {
            return Err("That square is taken");
        }

        return Ok((x_coord_value, y_coord_value));
    }
}

fn display_tic_tac_toe(current_game: &Vec<[&str; 3]>) {
    for row in current_game {
        for square in row {
            print!("[{square}]");
        }
        println!();
    }
}

fn fill_progress(guess: &str, mut progress: Vec<String>, word: &str) -> Vec<String> {
    let mut letters: Vec<&str> = word.split("").collect();
    letters.pop();
    letters.remove(0);

    for i in 0..letters.len() {
        if letters[i] == guess {
            progress[i] = guess.to_string();
        }
    }

    progress
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
