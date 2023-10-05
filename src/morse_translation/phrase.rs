use std::fmt;
use crate::morse_translation::word::*;

#[derive(PartialEq, Debug)]
pub struct Phrase(Vec<Word>);

impl Phrase {
    fn new(words: Vec<Word>) -> Self {
        Phrase(words)
    }

    pub fn from(string_phrase: &str) -> Phrase {
        let words = string_phrase.split_ascii_whitespace();
        let mut phrase = vec![];

        for word in words {
            let new_word = Word::from(word);

            phrase.push(new_word);
        }

        let new_phrase = Phrase::new(phrase);

        println!("{:?}", new_phrase);

        new_phrase
    }

    pub fn to_morse_string(&self) -> String {
        let mut morse_string: String = String::new();

        for (i, word) in self.0.iter().enumerate() {
            if i > 0 {
                morse_string.push_str(" / ");
            }
            morse_string.push_str(&word.to_morse_string());
        }

        morse_string
    }

    pub fn translate(&self) -> () {
        println!("Morse string: {}", self);
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_morse_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Phrase;
    use crate::morse_translation::{letter::{Letter, Symbol}, word::Word};
    
    #[test]
    fn compose_phrase() {
        let string: &str = "CIAO";
        let phrase = Phrase::from(&string);
        
        let letters = vec![
            Letter::new(vec![Symbol::Line, Symbol::Dot, Symbol::Line, Symbol::Dot]),
            Letter::new(vec![Symbol::Dot, Symbol::Dot]),
            Letter::new(vec![Symbol::Dot, Symbol::Line]),
            Letter::new(vec![Symbol::Line, Symbol::Line, Symbol::Line])
        ];

        let phrase_translation = Phrase::new(vec!(Word::new(letters)));

        assert_eq!(phrase.to_morse_string(), "-.-. .. .- ---");
        assert_eq!(phrase, phrase_translation);
    }
}