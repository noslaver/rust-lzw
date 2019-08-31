use std::collections::HashMap;
// use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Default)]
pub struct Encoder {
    codes: HashMap<Vec<u8>, u32>,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            codes: HashMap::new(),
        }
    }

    pub fn encode_bytes(&mut self, buf: &[u8]) -> Vec<u32> {
        // initialize codes dictionary
        for n in 0..=255 {
            self.codes.insert(vec![n], u32::from(n));
        }
        let mut next_code = 257;

        let mut output = Vec::new();
        let mut string = Vec::new();

        for &c in buf {
            string.push(c);
            if !self.codes.keys().any(|k| k.starts_with(&string[..])) {
                // insert new code
                self.codes.insert(string.clone(), next_code);
                next_code += 1;

                // output code for string up to the last character
                string.pop();
                output.push(*self.codes.get(&string).expect("This can't be None!"));

                // reset current string
                string.clear();
                string.push(c);
            }
        }
        output.push(*self.codes.get(&string).expect("This can't be None!"));

        output
    }
}

#[derive(Default)]
pub struct Decoder {
    strings: HashMap<u32, Vec<u8>>,
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            strings: HashMap::new(),
        }
    }

    pub fn decode_bytes(&mut self, buf: &[u32]) -> Vec<u8> {
        for n in 1..=255 {
            self.strings.insert(n, vec![n as u8]);
        }

        let mut output = Vec::new();

        // pointer to previous decoded string
        let mut prev_string: Option<Vec<u8>> = None;
        let mut next_code = 257;

        for code in buf {
            if self.strings.get(code).is_none() {
                if let Some(prev_string) = prev_string.clone() {
                    let mut string = prev_string.clone();
                    string.push(string[0]);
                    self.strings.insert(*code, string);
                }
            }

            let string = self.strings.get(code).expect("Can't be None");
            for &c in string {
                output.push(c);
            }

            if let Some(prev_string) = prev_string {
                let mut value = prev_string.clone();
                value.push(self.strings.get(code).expect("Can't be None")[0]);

                self.strings.insert(next_code, value);
                next_code += 1;
            }
            prev_string = Some(self.strings.get(code).expect("Can't be None").clone());
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::{Decoder, Encoder};

    #[test]
    fn it_encodes() {
        // arrange
        let mut encoder = Encoder::new();

        // act
        let out = encoder.encode_bytes(b"ABBABBBABBA");

        // assert
        assert_eq!(out, vec![65, 66, 66, 257, 258, 260, 65]);
    }

    #[test]
    fn it_decodes() {
        let mut decoder = Decoder::new();

        let out = decoder.decode_bytes(&[65, 66, 66, 257, 258, 260, 65]);

        assert_eq!(out, b"ABBABBBABBA");
    }

    #[test]
    fn edge_case() {
        let mut decoder = Decoder::new();

        let out = decoder.decode_bytes(&[65, 66, 257, 259]);

        assert_eq!(out, b"ABABABA");
    }

    #[test]
    fn it_round_trips() {
        let mut encoder = Encoder::new();

        let out = encoder.encode_bytes(b"ABBABBBABBA");

        assert_eq!(out, vec![65, 66, 66, 257, 258, 260, 65]);

        let mut decoder = Decoder::new();

        let out = decoder.decode_bytes(out.as_slice());

        assert_eq!(out, b"ABBABBBABBA");
    }
}
