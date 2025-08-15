use std::io::{self, Write, stdout};
use std::thread;
use std::time::Duration;

pub fn input(prefix: &str) -> String {
    print!("{prefix}");

    io::stdout().flush().unwrap();

    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error: {e}"),
    }

    user_input.trim().to_string()
}

pub fn clear_console() {
    print!("\x1b[2J\x1b[0;0H");
    stdout().flush().unwrap();
}

pub fn pause_for(secs: u64) {
    let pause_duration = Duration::from_secs(secs);
    thread::sleep(pause_duration);
}
