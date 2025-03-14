//! The `murcielago` module provides functionality for encoding and decoding messages
//! using the "MURCIELAGO" cipher, where specific letters are substituted with digits.
//!
//! # Overview
//! The MURCIELAGO cipher is a simple letter-to-digit substitution system where each
//! letter in the word "MURCIELAGO" is replaced by a number, either using a 0-based
//! or 1-based numbering scheme. The mappings are as follows:
//!
//! ## Base 0 Encoding:
//! ```text
//! M -> 0, U -> 1, R -> 2, C -> 3, I -> 4, E -> 5, L -> 6, A -> 7, G -> 8, O -> 9
//! ```
//!
//! ## Base 1 Encoding:
//! ```text
//! M -> 1, U -> 2, R -> 3, C -> 4, I -> 5, E -> 6, L -> 7, A -> 8, G -> 9, O -> 0
//! ```
//!
//! This module provides functions to encode and decode messages using both numbering bases.

/// Represents a coded message that can be decoded or encoded using the MURCIELAGO cipher.
pub struct Murcielago {
    message: String,
}

impl Murcielago {
    /// Creates a new `Murcielago` instance with the given message.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    /// Decodes the message using the MURCIELAGO cipher with a base-0 mapping.
    ///
    /// # Example
    /// ```
    /// use scout_encoder::murcielago::Murcielago;
    ///
    /// let cipher = Murcielago::new("9 8 7 6 5 4 3 2 1 0");
    /// assert_eq!(cipher.decode_base0(), "O G A L E I C R U M");
    /// ```
    pub fn decode_base0(&self) -> String {
        self.message
            .to_uppercase()
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
            .collect()
    }

    /// Decodes the message using the MURCIELAGO cipher with a base-1 mapping.
    ///
    /// # Example
    /// ```
    /// use scout_encoder::murcielago::Murcielago;
    ///
    /// let cipher = Murcielago::new("0 9 8 7 6 5 4 3 2 1");
    /// assert_eq!(cipher.decode_base1(), "O G A L E I C R U M");
    /// ```
    pub fn decode_base1(&self) -> String {
        self.message
            .to_uppercase()
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
            .collect()
    }

    /// Encodes the message using the MURCIELAGO cipher with a base-0 mapping.
    ///
    /// # Example
    /// ```
    /// use scout_encoder::murcielago::Murcielago;
    ///
    /// let cipher = Murcielago::new("MURCIELAGO");
    /// assert_eq!(cipher.encode_base0(), "0123456789");
    /// ```
    pub fn encode_base0(&self) -> String {
        self.message
            .to_uppercase()
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
            .collect()
    }

    /// Encodes the message using the MURCIELAGO cipher with a base-1 mapping.
    ///
    /// # Example
    /// ```
    /// use scout_encoder::murcielago::Murcielago;
    ///
    /// let cipher = Murcielago::new("MURCIELAGO");
    /// assert_eq!(cipher.encode_base1(), "1234567890");
    /// ```
    pub fn encode_base1(&self) -> String {
        self.message
            .to_uppercase()
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
            .collect()
    }
}

/// Decodes a message using the MURCIELAGO cipher with a base-0 mapping.
pub fn decode_base0(message: &str) -> String {
    Murcielago::new(message).decode_base0()
}

/// Decodes a message using the MURCIELAGO cipher with a base-1 mapping.
pub fn decode_base1(message: &str) -> String {
    Murcielago::new(message).decode_base1()
}

/// Encodes a message using the MURCIELAGO cipher with a base-0 mapping.
pub fn encode_base0(message: &str) -> String {
    Murcielago::new(message).encode_base0()
}

/// Encodes a message using the MURCIELAGO cipher with a base-1 mapping.
pub fn encode_base1(message: &str) -> String {
    Murcielago::new(message).encode_base1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base0() {
        assert_eq!(decode_base0("9 8 7 6 5 4 3 2 1 0"), "O G A L E I C R U M");
    }

    #[test]
    fn test_decode_base1() {
        assert_eq!(decode_base1("0 9 8 7 6 5 4 3 2 1"), "O G A L E I C R U M");
    }

    #[test]
    fn test_encode_base0() {
        assert_eq!(encode_base0("M U R C I E L A G O"), "0 1 2 3 4 5 6 7 8 9");
    }

    #[test]
    fn test_encode_base1() {
        assert_eq!(encode_base1("M U R C I E L A G O"), "1 2 3 4 5 6 7 8 9 0");
    }
}
