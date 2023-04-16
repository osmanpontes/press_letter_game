use rand::Rng;
use std::io::{self, Stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn generate_letter(last_target_char_option: Option<char>) -> char {
    let min = b'A';
    let max = b'Z';

    let random_char = rand::thread_rng().gen_range(min..=max) as char;

    if let Some(last_target_char) = last_target_char_option {
        if random_char == last_target_char {
            return generate_letter(last_target_char_option);
        }
    }

    return random_char;
}

fn print_game(stdout: &mut Stdout, target_char: &char, target_history: &str) {
    write!(
        stdout,
        "{}{}{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        ("PRESS LETTER: ".to_string() + target_char.to_string().as_str()).as_str(),
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
