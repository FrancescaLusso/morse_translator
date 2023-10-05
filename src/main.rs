use std::io;
mod morse_translation;

fn main() {
    let mut user_input: String = String::new();

    loop {
        println!("Enter some text: ");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                break
            }
            Err(err) => {
                println!("There was an error: {err}\nPlease try again");
                continue

            }
        }
    }
    morse_translation::translate(&mut user_input);
}

