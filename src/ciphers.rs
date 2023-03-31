use std::hash::Hash;
use crate::tools::Text;

pub mod monoalphabetic;
pub mod vinegere;

pub trait Cipher: Clone + Hash + Eq + PartialEq + Ord + PartialOrd{

    fn encrypt(&self, plaintext: &Text) -> Text;
    fn decrypt(&self, ciphertext: &Text) -> Text;

    fn branch(&self) -> Self { self.clone() }
}
