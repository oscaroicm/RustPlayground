use std::fs::{self, File};
use std::io::prelude::*;

use crate::utilities;

pub fn help() {
    let available_commands = [
        "c [note_name]",
        "r [note_name]",
        "u [note_name]",
        "d [note_name]",
        "ln - List notes",
    ];
    println!("Available commands");
    for command in available_commands {
        println!("{command}");
    }
}

fn format_to_path(mut note_name: String) -> String {
    note_name.push_str(".txt");
    note_name.insert_str(0, "./notes_db/");
    note_name
}

pub fn create_note(args: &Vec<&str>) {
    let note_name = format_to_path(args.join(" "));

    match fs::exists(&note_name) {
        Ok(true) => {
            println!("A note with that name already exists");
            return;
        }
        Ok(false) => (),
        Err(e) => panic!("Error checking file existance: {e}"),
    };

    let mut new_note = match File::create(note_name) {
        Ok(file) => file,
        Err(_) => panic!("Unexpected error"),
    };

    let content = utilities::input("Writing> ");
    match new_note.write_all(&content.as_bytes()) {
        Ok(_) => println!("New note created successfully"),
        Err(e) => println!("There was an error writing into the file: {e}"),
    }
}

pub fn read_note(args: &Vec<&str>) {
    let note_name = format_to_path(args.join(" "));

    let mut note_to_read = match File::open(note_name) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    let mut contents = String::new();
    match note_to_read.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => println!("Error: {e}"),
    }
    println!("{contents}");
}

pub fn update_note(args: &Vec<&str>) {
    let note_name = format_to_path(args.join(" "));

    let mut new_note = match File::create(note_name) {
        Ok(file) => file,
        Err(_) => panic!("Unexpected error"),
    };

    let content = utilities::input("Writing> ");
    match new_note.write_all(&content.as_bytes()) {
        Ok(_) => println!("Note updated successfully"),
        Err(e) => println!("There was an error writing into the file: {e}"),
    }
}

pub fn delete_note(args: &Vec<&str>) {
    let note_name = format_to_path(args.join(" "));

    match fs::remove_file(note_name) {
        Ok(_) => println!("Note deleted"),
        Err(e) => println!("Unable to remove note: {e}"),
    }
}

pub fn list_notes() {
    let notes: Vec<_> = match fs::read_dir("./notes_db") {
        Ok(entries) => entries,
        Err(_) => {
            println!("Error reading notes");
            return;
        }
    }
    .collect();

    if notes.len() == 0 {
        println!("No notes found");
    }

    for note in notes {
        if let Ok(note) = note {
            print!(
                "{}  ",
                note.file_name().into_string().unwrap().replace(".txt", "")
            );
        }
    }
    println!();
}
