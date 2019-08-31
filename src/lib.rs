use byteorder::{BigEndian, WriteBytesExt};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Default)]
pub struct Encoder {
    codes: HashMap<String, u32>,
}

impl Encoder {
    // fn encode<R, W>(reader: BufReader<R>, writer: BufWriter<W>)
    // where
        // R: Read,
        // W: Write,
    // {
    // }

    fn encode_bytes(self, buf: &[u8]) -> std::io::Result<Vec<u8>> {
        // TODO - initialize codes dictionary
        let mut output = Vec::new();

        let mut string = String::new();

        for c in buf {
            let c = *c as char;
            string.push(c);
            println!("str - {:?}", string);
            if !self.codes.keys().any(|k| k.starts_with(&string)) {
                string.pop();
                output.write_u32::<BigEndian>(*self.codes.get(&string).expect("This can't be None!"))?;
                string.clear();
                string.push(c);
            }
        }
        output.write_u32::<BigEndian>(*self.codes.get(&string).expect("This can't be None!"))?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::Encoder;

    #[test]
    fn it_works() {
        let mut encoder = Encoder::default();

        encoder.codes.insert(String::from("AB"), 0x0001_1011);
        encoder.codes.insert(String::from("B"), 0x1110_0100);

        let out = encoder.encode_bytes(String::from("ABB").as_bytes()).unwrap();
        assert_eq!(out, vec![0x00, 0x01, 0x10, 0x11, 0x11, 0x10, 0x01, 0x00]);
    }
}
