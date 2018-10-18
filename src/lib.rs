// A Rust port of [https://www.multicians.org/thvv/gpw.js]

extern crate ndarray;
extern crate rand;
#[macro_use]
extern crate serde;
extern crate serde_json;
mod trigram;

use rand::prelude::*;
use trigram::create_trigrams;

const LENGTH: usize = 10;
const THRESHOLD: f64 = 125729.;
const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub struct PasswordGenerator {
    pik : f64, // RANDOM 0-1
    ranno : f64,
    sum : u32,
    trigram: ndarray::Array3<u16>,
    output: String
}

impl Iterator for PasswordGenerator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.start();
        if self.pik == 0. && self.ranno == 0. {
            None
        } else {
            Some(self.calculate_next())
        }
    }
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        let mut retval = PasswordGenerator {
            pik: 0., 
            ranno: 0.,
            sum: 0,
            trigram: create_trigrams(),
            output: String::with_capacity(LENGTH)
        };

        retval
    }
}

fn to_character(index : usize) -> char {
    ALPHABET.get(index..index+1).unwrap().chars().next().unwrap()
}

fn index_at(value: &str, idx: usize) -> usize {
    let c = value.get(idx..idx+1).unwrap().chars().next().unwrap() as u8;

    (c - ('a' as u8)) as usize
}

impl PasswordGenerator {
    fn start(&mut self) {
        self.output.clear();
        self.pik = random::<f64>();
        self.ranno = self.pik * THRESHOLD;
        self.sum = 0;
        self.get_starting_point();
    }

    fn get_starting_point(&mut self) {
        for c1 in 0..26 {
            for c2 in 0..26 {
                for c3 in 0..26 {
                    self.sum += self.trigram[[c1, c2, c3]] as u32;
                    if self.sum as f64 > self.ranno {
                        self.output.push(to_character(c1));
                        self.output.push(to_character(c2));
                        self.output.push(to_character(c3));
                        return;
                    }
                }
            }
        }
    }

    fn calculate_next(&mut self) -> String {
        let mut nchar = self.output.len();
        while (nchar < LENGTH) {

            let c1 = index_at(&self.output, nchar-2);
            let c2 = index_at(&self.output, nchar-1);

            self.sum = 0;
            for c3 in 0..26 {
                self.sum += self.trigram[[c1, c2, c3]] as u32;
            }
            if self.sum == 0 {
                return "".to_owned()
            }

            self.pik = random::<f64>();
            self.ranno = self.pik * self.sum as f64;
            self.sum = 0;

            for c3 in 0..26 {
                self.sum += self.trigram[[c1, c2, c3]] as u32;
                if self.sum as f64 > self.ranno {
                    self.output.push(to_character(c3));
                    break;
                }
            }
            nchar += 1;
        }
        self.output.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut generator = PasswordGenerator::default();
        let password1 = generator.next().unwrap();
        let password2 = generator.next().unwrap();
        println!("{} / {}", password1, password2);

        assert!(password1 != password2);
    }
}
