use ciphers::{monoalphabetic::Monoalphabetic, Cipher};
use tools::Text;

use crate::cryptanalysis::{index_of_conicidence, NGrams, MonoFrequency};

mod cryptanalysis;
mod key_search;
mod tools;
mod ciphers;
mod traits;

fn main() {
    let text = Text::from_file("data/test1.txt".to_owned()).unwrap();
    let ceasar = Monoalphabetic::ceasar(2);

    println!("{:?}", index_of_conicidence(&text));

    let n = NGrams::from_file::<4>("english_quadgrams.txt");
    let p = n.p("QKPC");
    println!("{p}");

    let freqs = MonoFrequency::from_text(&text);
    println!("{}", freqs.entropy(&(0..26).collect::<Vec<u8>>().try_into().unwrap()));
    
    let new = ceasar.encrypt(&text);
    let freqs = MonoFrequency::from_text(&new);
    let score = |c: &Monoalphabetic| freqs.entropy_rel(&c.0);
    let mut tabu = key_search::Tabu::default();

    for i in 0..200 {
        tabu.step(&ceasar, 0.1, 20_000, score);
        let (k, v) = tabu.best.clone().unwrap();
        if i % 10 == 9 {
            println!("{} {v}", k.key_repr());
        }
    }


    println!("{}", tabu.best_n(30)
        .iter()
        .map(|x| format!("Key: {}, Score: {:?}", x.0.key_repr(), x.1))
        .collect::<Vec<_>>()
        .join("\r\n")
    );

}