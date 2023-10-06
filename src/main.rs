use std::io;

fn main() {
    let mut user_input: String = String::new();

    loop {
        println!("Enter some text: ");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                break
            }
            Err(err) => {
                println!("There was an error: {err}. Please try again!");
                continue

            }
        }
    }

    morse_translator::translate(&user_input);
}

