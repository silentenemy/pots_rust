use std::cmp::min;
use std::io::{Read, Result};

struct RotDecoder<R> {
    input: R,
    rot: u8,
}

// Сделайте трейт Read для RotDecoder.

fn match_char(c: u8, rot: u8) -> u8 {
    if (c as char).is_ascii_alphabetic() {
        let a = if (c as char).is_ascii_lowercase() {
            b'a'
        } else {
            b'A'
        };
        return (((c - a) + rot) % 26) + a;
    }
    else {
        return c;
    }
}

impl Read for RotDecoder<&[u8]>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut ctr: usize = 0;
        for i in 0..min(self.input.len(), buf.len()) {
            //buf[i] = self.input[i]-self.rot;
            buf[i] = match_char(self.input[i], self.rot);
            ctr += 1;
        }
        self.input = &[0; 0];
        Ok(ctr)
    }
}

fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
