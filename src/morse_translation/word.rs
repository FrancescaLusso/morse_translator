use crate::morse_translation::letter::Letter;

#[derive(PartialEq, Debug, Clone)]
pub struct Word(Vec<Letter>);

impl Word{
    pub fn new(letters: Vec<Letter>) -> Self {
        Word(letters)
    }

    pub fn from(word: &str) -> Word {
        let mut words = vec![];

        for c in word.chars() {
            match Letter::from(c) {
                Some(letter) => words.push(letter),
                None => continue
            }
        }
        Word::new(words)
    }

    pub fn to_morse_string(&self) -> String {
        let mut morse_string: String = String::new();

        for (i, letter) in self.0.iter().enumerate() {
            if i > 0 {
                morse_string.push(' ');
            }

            morse_string.push_str(&letter.to_morse_string());
        }
        morse_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morse_translation::letter::{Letter, Symbol};
    #[test]
    fn build_word() {
        let word = Word::from("CIAO");
        let letters = vec![
            Letter::new(vec![Symbol::Line, Symbol::Dot, Symbol::Line, Symbol::Dot]),
            Letter::new(vec![Symbol::Dot, Symbol::Dot]),
            Letter::new(vec![Symbol::Dot, Symbol::Line]),
            Letter::new(vec![Symbol::Line, Symbol::Line, Symbol::Line])
        ];

        let word_translation = Word::new(letters);

        assert_eq!(word.to_morse_string(), "-.-. .. .- ---");
        assert_eq!(word, word_translation);
    }
}
