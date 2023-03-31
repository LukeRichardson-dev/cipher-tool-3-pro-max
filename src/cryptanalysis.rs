use std::{fs::File, io::Read};
use crate::tools::Text;

mod ngrams;
pub use ngrams::*;

mod frequency;
pub use frequency::*;

const LATIN_ALPHABET_NORMALISATION_COEFFICIENT: usize = 26;

pub const ENGLISH_IC: f64 = 1.73; 

pub fn index_of_conicidence(text: &Text) -> f64 {
    let tokens: Vec<u8> = text.iter().map(|x| *x).collect();
    let mut c = [0; 26];
    tokens.iter().for_each(|&x| c[x as usize] += 1);

    let total: i32 = c.iter().map(|&n| n * (n - 1)).sum();
    let den: f64 = (tokens.len() * (tokens.len() - 1)) as f64;

    (total * LATIN_ALPHABET_NORMALISATION_COEFFICIENT as i32) as f64 / den
}
