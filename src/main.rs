use chrono::prelude::*;
use indexmap::IndexMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod colorization;
mod games;
mod notes;
mod utilities;
use colorization::Color;

fn main() {
    utilities::clear_console();
    println!(
        "{}",
        "Playground - Built with Rust - Type 'help' to see all available commands".green()
    );

    // Complex implementation, using concurrency
    let seconds_passed = Arc::new(Mutex::new(0));
    {
        let c_seconds_passed = seconds_passed.clone();
        thread::spawn(move || {
            loop {
                let mut lock = c_seconds_passed.lock().unwrap();
                *lock += 1;
                drop(lock);

                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // Straightforward implementation for a timer
    let now = Instant::now();

    'main_loop: loop {
        let user_input = utilities::input("> ");
        match user_input.as_str() {
            "help" => show_help(),
            "exit" => break 'main_loop,
            "date" => print_date(),
            "play" => game_choice(),
            "notes" => notes_crud(),
            "timer" => get_session_time(Arc::clone(&seconds_passed)),
            "elapsed" => show_elapsed_time(&now),
            "colors" => println!(
                "{} - {} - {} - {} - {} - {}",
                "Red".red(),
                "Green".green(),
                "Blue".blue(),
                "Yellow".yellow(),
                "Purple".purple(),
                "Cyan".cyan()
            ),
            "clear" => utilities::clear_console(),
            &_ => println!("Command not found"),
        }
    }
}

fn notes_crud() {
    utilities::clear_console();
    println!(
        "{}",
        "Notes - Type 'help' to see all available commands".yellow()
    );

    let _ = fs::create_dir("notes_db");

    let with_arguments = ["c", "r", "u", "d"];

    loop {
        let user_input = utilities::input("Notes> ");

        let command: Vec<&str> = user_input.split(' ').collect();
        if command.len() < 2 && with_arguments.contains(&command[0]) {
            println!("Invalid command");
            continue;
        }
        let mut arguments: Vec<&str> = Vec::new();
        command[1..].clone_into(&mut arguments);

        match command[0] {
            "help" => notes::help(),
            "exit" => {
                utilities::clear_console();
                println!(
                    "{}",
                    "Playground - Built with Rust - Type 'help' to see all available commands"
                        .green()
                );
                return;
            }
            "c" => notes::create_note(&arguments),
            "r" => notes::read_note(&arguments),
            "u" => notes::update_note(&arguments),
            "d" => notes::delete_note(&arguments),
            "ln" => notes::list_notes(),
            _ => println!("Command not found"),
        }
    }
}

fn game_choice() {
    let available_games = [
        "Rock Paper Scissors",
        "Guess The Number",
        "Hangman",
        "Tic Tac Toe",
    ];
    utilities::clear_console();

    println!("{}", "Available Games".blue());

    for (i, game) in available_games.iter().enumerate() {
        println!("[{}] {game}", i + 1);
    }

    let user_input = utilities::input("Play [enter a number]: ");
    match user_input.as_str() {
        "1" => games::start_rock_paper_scissors(),
        "2" => games::start_guess_the_number(),
        "3" => games::start_hangman(),
        "4" => games::start_tic_tac_toe(),
        &_ => println!("Game not found"),
    }
}

fn print_date() {
    let local_time: DateTime<Local> = Local::now();
    println!("{local_time:?}");
}

fn get_session_time(time: Arc<Mutex<u64>>) {
    let total_secs = *time.lock().unwrap();
    println!("Time passed: {}", format_secs(total_secs));
}

fn format_secs(time: u64) -> String {
    let hours = time / 3600;
    let minutes = time / 60;
    let seconds = time - (hours * 3600) - (minutes * 60);
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn show_elapsed_time(start: &Instant) {
    let elapsed_time = start.elapsed();
    println!("Elapsed: {}", format_secs(elapsed_time.as_secs()));
}

fn show_help() {
    let commands = IndexMap::from([
        ("exit", "Exits the program"),
        ("date", "Returns the local date and time"),
        ("play", "Play a game"),
        ("notes", "Starts \"Notes\""),
        ("colors", "Displays a list of colors"),
        (
            "timer",
            "See how much time has passed since the program was opened",
        ),
        ("elapsed", "Alternative to 'timer'"),
        ("clear", "Clear the console"),
    ]);
    for (command, description) in commands {
        println!("{command} - {description}");
    }
}
