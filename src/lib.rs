pub mod morse_translation;
use crate::morse_translation::phrase::Phrase;

pub fn translate(user_input: &String) -> () {

    let formatted_input: String = user_input.trim().to_uppercase();
    Phrase::from(&formatted_input).translate();
}