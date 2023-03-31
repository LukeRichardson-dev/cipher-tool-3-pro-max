use crate::tools::Text;

pub const FREQUENCIES: [f64; 26] = include!("../static/frequencies.ron");
pub const ENTROPY: f64 = 4.199118103393031;

pub struct MonoFrequency([f64; 26]);

impl MonoFrequency {
    pub fn from_text(text: &Text) -> Self {
        let counts = text.iter().fold([0; 26], |mut acc, x| {
            acc[*x as usize] += 1;
            acc
        });
        let total = counts.into_iter().sum::<usize>() as f64;
        Self(counts.map(|x| (x + 1) as f64 / total))
    }

    pub fn entropy_rel(&self, key: &[u8; 26]) -> f64 {
        (-ENTROPY - self.entropy(key)).abs()
    }
    
    pub fn entropy(&self, key: &[u8; 26]) -> f64 {
        key.iter().enumerate().map(|(i, &x)| 
            FREQUENCIES[i].log2() * self.0[x as usize]
        ).sum::<f64>()
    }
}
