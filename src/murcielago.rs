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
        let lowercase_code = self.coded_message.to_lowercase();

        let decoded = lowercase_code
            .chars()
            .map(|c| match c {
                '0' => 'm',
                '1' => 'u',
                '2' => 'r',
                '3' => 'c',
                '4' => 'i',
                '5' => 'e',
                '6' => 'l',
                '7' => 'a',
                '8' => 'g',
                '9' => 'o',
                _ => c,
            })
            .collect();

        decoded
    }

    /// Decodes the message, replacing numbers with their corresponding letters using 1 as it's base.
    pub fn decode_message_base1(&self) -> String {
        let lowercase_code = self.coded_message.to_lowercase();

        let decoded = lowercase_code
            .chars()
            .map(|c| match c {
                '1' => 'm',
                '2' => 'u',
                '3' => 'r',
                '4' => 'c',
                '5' => 'i',
                '6' => 'e',
                '7' => 'l',
                '8' => 'a',
                '9' => 'g',
                '0' => 'o',
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
    let lowercase_message = message.to_lowercase();

    let encoded = lowercase_message
        .chars()
        .map(|c| match c {
            'm' => '0',
            'u' => '1',
            'r' => '2',
            'c' => '3',
            'i' => '4',
            'e' => '5',
            'l' => '6',
            'a' => '7',
            'g' => '8',
            'o' => '9',
            _ => c,
        })
        .collect();

    encoded
}

/// Encodes a given string using the `Murcielago` cipher starting with 1 without requiring an instance
pub fn encode_base1(message: &str) -> String {
    let lowercase_message = message.to_lowercase();

    let encoded = lowercase_message
        .chars()
        .map(|c| match c {
            'm' => '1',
            'u' => '2',
            'r' => '3',
            'c' => '4',
            'i' => '5',
            'e' => '6',
            'l' => '7',
            'a' => '8',
            'g' => '9',
            'o' => '0',
            _ => c,
        })
        .collect();

    encoded
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_scouts_base0() {
        assert_eq!(decode_base0("S391TS"), "scouts");
    }

    #[test]
    fn should_return_scouts_base1() {
        assert_eq!(decode_base1("S402TS"), "scouts");
    }

    #[test]
    fn should_return_s391ts_from_scouts() {
        assert_eq!(encode_base0("SCOUTS"), "s391ts");
    }

    #[test]
    fn should_return_s402ts_from_scouts() {
        assert_eq!(encode_base1("SCOUTS"), "s402ts");
    }
}
