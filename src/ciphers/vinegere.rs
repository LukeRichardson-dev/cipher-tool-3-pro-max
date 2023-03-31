use crate::tools::Text;

use super::Cipher;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vinegere {
    key: Vec<u8>,
}

impl Vinegere {
    pub fn from_key(key: Vec<u8>) -> Self {
        Self { key }
    }

    pub fn from_text(text: &Text) -> Self {
        let mut i = 2;
        loop {
            let t = text.chunked(i);
            
            i += 1;
        }
    }
}

impl Cipher for Vinegere {
    fn branch(&self) -> Self {
        let mut n = self.clone();
        let t: usize = rand::random();
        for _ in 0..(t & 0b11) {
            let a: usize = rand::random();
            let i = a >> 1 % self.key.len();
            n.key[i] = ((a + if a & 1 == 1 {
                1
            } else {
                25
            }) % 26) as u8;
        }
        n
    }

    fn encrypt(&self, plaintext: &Text) -> Text {
        Text::from_bytes(plaintext.iter()
            .enumerate()
            .map(|(i, x)| (x + self.key[i & self.key.len()]) % 26)
            .collect())
    }

    fn decrypt(&self, ciphertext: &Text) -> Text {
        Text::from_bytes(ciphertext.iter()
            .enumerate()
            .map(|(i, x)| (x - self.key[i & self.key.len()]) % 26)
            .collect())
    }
}
