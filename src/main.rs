use std::{env, io, process, thread, time::Duration};
use crossterm::*;
use random_string::{
    charsets::ALPHANUMERIC,
    generate,
};
use rand::Rng;

fn main() {
    let Some(password) = env::args().nth(1) else {
        eprintln!("Usage: hacker <password>");
        process::exit(1);
    };

    let mut rng = rand::thread_rng();

    execute!(
        io::stdout(),
        style::SetForegroundColor(style::Color::Blue),
        style::Print("Hacking password...\n"),
        style::SetForegroundColor(style::Color::Red),
    ).unwrap();
    for i in 0..password.len() {
        for _ in 0..rng.gen_range(0..32) {
            let s1 = &password[0..i];
            let s2 =generate(password.len() - i, ALPHANUMERIC);
            execute!(
                io::stdout(),
                cursor::SavePosition,
                style::SetForegroundColor(style::Color::Green),
                style::Print(s1),
                style::SetForegroundColor(style::Color::Red),
                style::Print(s2),
                cursor::RestorePosition,
            ).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    }

    execute!(
        io::stdout(),
        style::SetForegroundColor(style::Color::Green),
        style::SetBackgroundColor(style::Color::DarkGrey),
        style::Print(format!("{password}\n")),
        style::SetForegroundColor(style::Color::Blue),
        style::SetBackgroundColor(style::Color::Reset),
        style::Print("Password successfully hacked.\n"),
    ).unwrap();
}
