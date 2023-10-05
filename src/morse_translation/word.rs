use crate::morse_translation::letter::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Word(Vec<Letter>);

impl Word{
    pub fn new(letters: Vec<Letter>) -> Self {
        Word(letters)
    }

    pub fn from(word: &str) -> Word {
        let mut new_word = vec![];

        for c in word.chars() {
            match Letter::from(c) {
                Some(letter) => new_word.push(letter),
                None => continue
            }
        }

        Word::new(new_word)
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

    #[test]
    fn build_word() {
        let word = Word::from("AA");
        let letters = vec![
            Letter::new(
                vec![Symbol::Dot, Symbol::Line]),
            Letter::new(
                vec![Symbol::Dot, Symbol::Line]
            )
        ];

        let word_symbols = Word::new(letters);

        assert_eq!(word, word_symbols);
        assert_eq!(word.to_morse_string(), ".- .-");
    }
}