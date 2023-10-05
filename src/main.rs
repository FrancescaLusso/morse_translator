use std::io;
mod morse_translation;
use crate::morse_translation::phrase::Phrase;

fn main() {
    let mut user_string: String = String::new();

    loop {
        println!("Enter some text: ");

        match io::stdin().read_line(&mut user_string) {
            Ok(_) => {
                break
            }
            Err(err) => {
                println!("There was an error: {err}\nPlease try again");
                continue

            }
        }
    }

    let user_string: String = user_string.trim().to_uppercase();

    Phrase::from(&user_string).translate();
}

