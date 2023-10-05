#[derive(PartialEq, Debug, Clone)]
pub enum Symbol {
    Dot,
    Line
}

#[derive(PartialEq, Debug, Clone)]
pub struct Letter(Vec<Symbol>);

impl Letter {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        Letter(symbols)
    }

    pub fn from(char: char) -> Option<Letter> {
        use Symbol::*;
    
        let symbols: Option<Vec<Symbol>> = match char {
            'A' => Some(vec![Dot, Line]),
            'B' => Some(vec![Line, Dot]),
            'C' => Some(vec![Line, Dot, Line, Dot]),
            'D' => Some(vec![Line, Dot, Dot]),
            'E' => Some(vec![Dot]),
            'F' => Some(vec![Dot, Dot, Line, Dot]),
            'G' => Some(vec![Line, Line, Dot]),
            'H' => Some(vec![Dot, Dot, Dot, Dot]),
            'I' => Some(vec![Dot, Dot]),
            'J' => Some(vec![Dot, Line, Line, Line]),
            'K' => Some(vec![Line, Dot, Line]),
            'L' => Some(vec![Dot, Line, Dot, Dot]),
            'M' => Some(vec![Line, Line]),
            'N' => Some(vec![Line, Dot]),
            'O' => Some(vec![Line, Line, Line]),
            'P' => Some(vec![Dot, Line, Line, Dot]),
            'Q' => Some(vec![Line, Line, Dot, Line]),
            'R' => Some(vec![Dot, Line, Dot]),
            'S' => Some(vec![Dot, Dot, Dot]),
            'T' => Some(vec![Line]),
            'U' => Some(vec![Dot, Dot, Line]),
            'V' => Some(vec![Dot, Dot, Dot, Line]),
            'W' => Some(vec![Dot, Line, Line]),
            'X' => Some(vec![Line, Dot, Dot, Line]),
            'Y' => Some(vec![Line, Dot, Line, Line]),
            'Z' => Some(vec![Line, Line, Dot, Dot]),
            '0' => Some(vec![Line, Line, Line, Line, Line]),
            '1' => Some(vec![Dot, Line, Line, Line, Line]),
            '2' => Some(vec![Dot, Dot, Line, Line, Line]),
            '3' => Some(vec![Dot, Dot, Dot, Line, Line]),
            '4' => Some(vec![Dot, Dot, Dot, Dot, Line]),
            '5' => Some(vec![Dot, Dot, Dot, Dot, Dot]),
            '6' => Some(vec![Line, Dot, Dot, Dot, Dot]),
            '7' => Some(vec![Line, Line, Dot, Dot, Dot]),
            '8' => Some(vec![Line, Line, Line, Dot, Dot]),
            '9' => Some(vec![Line, Line, Line, Line, Dot]),
            _other => None
        };

        if let Some(letter) = symbols {
            Some(Letter::new(letter))
        } else {
            None
        }
    }

    pub fn to_morse_string(&self) -> String {
        let mut morse_string: String = String::new();

        for symbol in &self.0 {
            let string: char = match symbol {
                Symbol::Dot => '.',
                Symbol::Line => '-',
            };
            morse_string.push(string)
        }
        morse_string
    }
}

#[cfg(test)]
mod tests {
    use super::{Symbol, Letter};

    #[test]
    fn build_letter() {
        let letter = Letter::from('A');
        let invalid_char = Letter::from(',');
        let symbols = Some(
            Letter::new(vec![Symbol::Dot, Symbol::Line])
        );

        assert_eq!(letter, symbols);
        assert!(invalid_char.is_none());
        assert_eq!(letter.unwrap().to_morse_string(), ".-")
    }
}