use chrono::prelude::*;
use indexmap::IndexMap;
use std::fs;

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

    'main_loop: loop {
        let user_input = utilities::input("> ");
        match user_input.as_str() {
            "help" => show_help(),
            "exit" => break 'main_loop,
            "date" => print_date(),
            "play" => game_choice(),
            "notes" => notes_crud(),
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

        let command: Vec<&str> = user_input.split(' ').collect(); // Transforms from the iterator
        // into a vector collection
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
    let available_games = ["Rock Paper Scissors", "Guess The Number", "Hangman"];
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
        &_ => println!("Game not found"),
    }
}

fn print_date() {
    let local_time: DateTime<Local> = Local::now();
    println!("{local_time:?}");
}

fn show_help() {
    let commands = IndexMap::from([
        ("exit", "Exits the program"),
        ("date", "Returns the local date and time"),
        ("play", "Play a game"),
        ("notes", "Starts \"Notes\""),
        ("colors", "Displays a list of colors"),
        ("clear", "Clear the console"),
    ]);
    for (command, description) in commands {
        println!("{command} - {description}");
    }
}
