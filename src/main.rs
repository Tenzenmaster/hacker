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

    for i in 0..password.len() {
        for _ in 0..rng.gen_range(0..16) {
            let mut str = password[0..i].to_owned();
            str.push_str(&generate(password.len() - i, ALPHANUMERIC));
            execute!(
                io::stdout(),
                cursor::SavePosition,
                style::Print(str),
                cursor::RestorePosition,
            ).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    }
    println!("{password}");
}
