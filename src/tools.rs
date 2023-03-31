use std::{string::FromUtf8Error, io::Read, slice::Iter};
use rand::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Text {
    tokens: Vec<u8>,
}

impl Text {
    pub fn from_string(buf: String) -> Text {
        Self {
            tokens: buf.to_uppercase()
                .bytes()
                .into_iter()
                .filter_map(|x| {
                    if x >= 65 && x <= 90 {
                        Some(x - 65)
                    } else {
                        None
                    }
                }).collect()
        }
    }

    pub fn from_file(path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?; 
        let mut buf = String::default();
        file.read_to_string(&mut buf)?;

        Ok(Self::from_string(buf))
    }

    pub fn from_bytes(buf: Vec<u8>) -> Self {
        Self {
            tokens: buf,
        }
    }

    pub fn to_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.tokens.iter().map(|x| x + 65).collect())
    }

    
    pub fn sample_text(&self, size: usize) -> Text {
        let i = random::<usize>() % (self.tokens.len() - size);
        Self { 
            tokens: self.tokens[i..(i + size)].to_vec() 
        }
    }

    pub fn sample_string(&self, size: usize) -> Result<String, FromUtf8Error> {
        self.sample_text(size).to_string()
    }

    pub fn iter<'a>(&self) -> Iter<u8> {
        self.tokens.iter()
    }

    pub fn chunked(&self, n: usize) -> Vec<Text> {
        (0..n).map(
            |i| {
                Text::from_bytes(self.iter().skip(i)
                    .step_by(n)
                    .map(|x| *x)
                    .collect())
            }
        ).collect()
    }
}