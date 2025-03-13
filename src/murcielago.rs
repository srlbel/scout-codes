/// The `murcielago` module provides functionality to decode a message
/// where numbers are substituted for letters based on the "MURCIELAGO" cipher.

/// Represents a coded message that can be decoded.
pub struct Murcielago {
    coded_message: String,
}

impl Murcielago {
    /// Creates a new `Murcielago` instance.
    pub fn new(coded_message: &str) -> Self {
        Self {
            coded_message: coded_message.to_string(),
        }
    }

    /// Decodes the message, replacing numbers with their corresponding letters using 0 as it's base.
    pub fn decode_message_base0(&self) -> String {
        let uppercase_code = self.coded_message.to_uppercase();

        let decoded = uppercase_code
            .chars()
            .map(|c| match c {
                '0' => 'M',
                '1' => 'U',
                '2' => 'R',
                '3' => 'C',
                '4' => 'I',
                '5' => 'E',
                '6' => 'L',
                '7' => 'A',
                '8' => 'G',
                '9' => 'O',
                _ => c,
            })
            .collect();

        decoded
    }

    /// Decodes the message, replacing numbers with their corresponding letters using 1 as it's base.
    pub fn decode_message_base1(&self) -> String {
        let uppercase_code = self.coded_message.to_uppercase();

        let decoded = uppercase_code
            .chars()
            .map(|c| match c {
                '1' => 'M',
                '2' => 'U',
                '3' => 'R',
                '4' => 'C',
                '5' => 'I',
                '6' => 'E',
                '7' => 'L',
                '8' => 'A',
                '9' => 'G',
                '0' => 'O',
                _ => c,
            })
            .collect();

        decoded
    }
}

/// Decodes a given string using the `Murcielago` cipher starting with 0 without requiring an instance.
pub fn decode_base0(coded_message: &str) -> String {
    Murcielago::new(coded_message).decode_message_base0()
}

/// Decodes a given string using the `Murcielago` cipher starting with 1 without requiring an instance.
pub fn decode_base1(coded_message: &str) -> String {
    Murcielago::new(coded_message).decode_message_base1()
}

/// Encodes a given string using the `Murcielago` cipher starting with 0 without requiring an instance
pub fn encode_base0(message: &str) -> String {
    let uppercase_message = message.to_uppercase();

    let encoded = uppercase_message
        .chars()
        .map(|c| match c {
            'M' => '0',
            'U' => '1',
            'R' => '2',
            'C' => '3',
            'I' => '4',
            'E' => '5',
            'L' => '6',
            'A' => '7',
            'G' => '8',
            'O' => '9',
            _ => c,
        })
        .collect();

    encoded
}

/// Encodes a given string using the `Murcielago` cipher starting with 1 without requiring an instance
pub fn encode_base1(message: &str) -> String {
    let uppercase_message = message.to_uppercase();

    let encoded = uppercase_message
        .chars()
        .map(|c| match c {
            'M' => '1',
            'U' => '2',
            'R' => '3',
            'C' => '4',
            'I' => '5',
            'E' => '6',
            'L' => '7',
            'A' => '8',
            'G' => '9',
            'O' => '0',
            _ => c,
        })
        .collect();

    encoded
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_string_from_encoded_base0() {
        assert_eq!(decode_base0("9 8 7 6 5 4 3 2 1 0"), "O G A L E I C R U M");
    }

    #[test]
    fn should_return_string_from_encoded_base1() {
        assert_eq!(decode_base1("0 9 8 7 6 5 4 3 2 1"), "O G A L E I C R U M");
    }

    #[test]
    fn should_return_encoded_base0_from_string() {
        assert_eq!(encode_base0("M U R C I E L A G O"), "0 1 2 3 4 5 6 7 8 9");
    }

    #[test]
    fn should_return_encoded_base1_from_string() {
        assert_eq!(encode_base1("M U R C I E L A G O"), "1 2 3 4 5 6 7 8 9 0");
    }
}
