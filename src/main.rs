use rand::Rng;
use std::io::{self, Stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

const MIN_ASCII_LETTER: u8 = b'A';
const MAX_ASCII_LETTER: u8 = b'Z';

fn generate_letter(last_target_char_option: Option<char>) -> char {
    let max = if last_target_char_option == None {
        MAX_ASCII_LETTER
    } else {
        MAX_ASCII_LETTER - 1
    };

    let random_char = rand::thread_rng().gen_range(MIN_ASCII_LETTER..=max) as char;

    if let Some(last_target_char) = last_target_char_option {
        if random_char >= last_target_char {
            return (random_char as u8 + 1) as char;
        }
    }

    return random_char;
}

fn print_game(stdout: &mut Stdout, target_char: &char, target_history: &str) {
    let message = format!("PRESS LETTER: {}", target_char);

    write!(
        stdout,
        "{}{}{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        message,
        termion::cursor::Goto(1, 3),
        target_history,
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut target_char = generate_letter(None);
    let mut target_history = String::new();

    print_game(&mut stdout, &target_char, &target_history);

    for pressed_key in stdin.keys() {
        match pressed_key.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(input_char) => {
                let input_char_uppercase = input_char.to_ascii_uppercase();
                if input_char_uppercase == target_char {
                    target_history.push(input_char_uppercase);
                    target_char = generate_letter(Some(target_char));
                    print_game(&mut stdout, &target_char, &target_history);
                }
            }
            _ => (),
        }
    }
}
