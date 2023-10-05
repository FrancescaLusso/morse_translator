mod letter;
mod word;
pub mod phrase;

use crate::morse_translation::phrase::Phrase;

pub fn translate(user_input: &mut String) {
    let user_input: String = user_input.trim().to_uppercase();

    Phrase::from(&user_input).translate();
}