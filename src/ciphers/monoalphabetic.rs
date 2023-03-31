use super::Cipher;
use crate::Text;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Monoalphabetic(pub [u8; 26]);

impl Monoalphabetic {
    pub fn ceasar(k: u8) -> Self {
        let mut buf = [0u8; 26];
        for i in 0u8..26u8 {
            buf[i as usize] = (i + k) % 26;
        }
        Self(buf)
    }

    pub fn key_repr(&self) -> String {
        self.0.iter().filter(|&&x| x < 26).map(|x| char::from(x + 65)).collect()
    }
}

impl Cipher for Monoalphabetic {
    fn encrypt(&self, plaintext: &Text) -> Text {
        Text::from_bytes(plaintext.iter().map(
            |x| self.0[*x as usize]
        ).collect())
    }

    fn decrypt(&self, ciphertext: &Text) -> Text {
        let mut t = [0u8; 26];
        self.0.into_iter().enumerate().for_each( |(i, x)| t[x as usize] = i as u8 ) ;

        Text::from_bytes(ciphertext.iter().map( |x| t[*x as usize] as u8 ).collect())
    }

    fn branch(&self) -> Self { 
        let mut n = self.clone();
        let t: usize = rand::random();
        for _ in 0..(t & 0b11) {
            let a: usize = rand::random();
            n.0.swap(a % 26, (a >> 8) % 26);
        }
        n
    }

}

