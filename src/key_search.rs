use std::{collections::{HashSet, HashMap}};

use crate::ciphers::Cipher;

#[derive(Debug)]
pub struct Tabu<K: Cipher> {
    attempts: HashMap<K, f64>,
    temp: f64,
    pub best: Option<(K, f64)>,
}

impl<K: Cipher> Default for Tabu<K> {
    fn default() -> Self {
        Self { 
            attempts: HashMap::default(), 
            temp: 20.0, 
            best: None,
        }
    }
}

impl<K: Cipher> Tabu<K>  {
    pub fn from_temp(t: f64) -> Self {
        Self { 
            attempts: HashMap::default(), 
            temp: t, 
            best: None,
        }
    }

    pub fn step<F: Fn(&K) -> f64>(
        &mut self, 
        start: &K, 
        decay: f64, 
        mut epochs: usize, 
        score: F
    ) -> f64 {
        let mut best = start.branch();
        let mut s = score(&best);
        if let None = self.best {
            self.best = Some((best.clone(), s))
        }

        
        while epochs != 0 {
            let nb = best.branch();
            if let Some(c) = self.attempts.get(&nb) {
                let df = s - c;
                let r: f64 = rand::random();
                if (df / self.temp).exp() > r {
                    s = *c;
                    best = nb;
                }

                continue;
            }

            let ns = score(&best);
            self.attempts.insert(nb.clone(), ns);

            let df = s - ns;

            if df >= 0.0 {
                best = nb.clone();
                s = ns;
                if s < self.best.as_ref().unwrap().1 {
                    self.best = Some((nb, ns));
                }
            } else {
                let r: f64 = rand::random();
                if (df / self.temp).exp() > r {
                    s = ns;
                    best = nb;
                }
            }
            
            epochs -= 1;
        }

        self.temp -= decay;

        if let Some(old) = &self.best {
            if s < old.1 {
                self.best = Some((best, s));
            }
        } else {
            self.best = Some((best, s));
        }

        s
    }

    pub fn best_n(&self, n: usize) -> Vec<(K, f64)> {
        let mut vals = self.attempts.iter().collect::<Vec<_>>();
        vals.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        vals[0..n].into_iter()
            .map(|&(a, b)| (a.clone(), b.clone()))
            .collect()
    }
}
