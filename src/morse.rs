pub struct Morse {
    message: String,
}

impl Morse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn decode(&self) -> String {
        let words = self.message.split("//");
        let mut decoded_message = String::new();

        for word in words {
            let mut decoded_word = String::new();
            for letter in word.split('/') {
                let decoded_letter = match letter {
                    ".-" => "A",
                    "-..." => "B",
                    "-.-." => "C",
                    "-.." => "D",
                    "." => "E",
                    "..-." => "F",
                    "--." => "G",
                    "...." => "H",
                    ".." => "I",
                    ".---" => "J",
                    "-.-" => "K",
                    ".-.." => "L",
                    "--" => "M",
                    "-." => "N",
                    "---" => "O",
                    ".--." => "P",
                    "--.-" => "Q",
                    ".-." => "R",
                    "..." => "S",
                    "-" => "T",
                    "..-" => "U",
                    "...-" => "V",
                    ".--" => "W",
                    "-..-" => "X",
                    "-.--" => "Y",
                    "--.." => "Z",
                    "-----" => "0",
                    ".----" => "1",
                    "..---" => "2",
                    "...--" => "3",
                    "....-" => "4",
                    "....." => "5",
                    "-...." => "6",
                    "--..." => "7",
                    "---.." => "8",
                    "----." => "9",
                    _ => panic!("Invalid morse code! {}", letter),
                };

                decoded_word.push_str(decoded_letter);
            }

            decoded_message.push_str(&decoded_word);
            decoded_message.push_str(" ");
        }

        decoded_message.trim().to_string()
    }

    pub fn encode(&self) -> String {
        todo!()
    }
}

pub fn decode(message: &str) -> String {
    Morse::new(message).decode()
}

pub fn encode(message: &str) -> String {
    Morse::new(message).encode()
}

#[cfg(test)]
mod test {
    use crate::morse::decode;

    #[test]
    fn should_return_a_bc() {
        assert_eq!(decode(".-//-.../-.-."), "A BC")
    }
}
