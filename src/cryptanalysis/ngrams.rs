use std::{fs::File, io::Read};


pub struct NGrams(Vec<f64>);

impl NGrams {
    pub fn from_file<const A: u32>(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = "".to_owned();
        file.read_to_string(&mut data).unwrap();

        let mut sum = 0.0;
        let mut buf = vec![0.0; 26usize.pow(A + 1)];
        for i in data.split("\n") {
            let mut parts = i.split(" ");
            let tag = parts.next().unwrap();
            let num = parts.next().unwrap().parse::<f64>().unwrap();
            sum += num;
            let k = Self::hash(tag);
            buf[k] = num;
        }

        let floor = 0.01 / sum; 
        for i in buf.iter_mut() {
            *i = (*i / sum + floor).log10();
        }

        Self(buf)
    }

    fn hash(key: &str) -> usize {
        let mut buf = 0;
        for (idx, dat) in key.as_bytes().into_iter().enumerate() {
            buf += POW26[idx] * *dat as usize;
        }
        buf
    } 

    pub fn p(&self, pat: &str) -> f64 {
        self.0[Self::hash(pat)]
    }
}

const POW26: [usize; 5] = [
    1,
    26,
    26usize.pow(2),
    26usize.pow(3),
    26usize.pow(4),
];

