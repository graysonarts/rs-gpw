use ndarray::Array3;
use serde_json;

const TRIGRAM_DATA : &'static str = include_str!("trigram.json");

pub fn create_trigrams() -> Array3<u16> {
    serde_json::from_str(TRIGRAM_DATA).unwrap()
}


#[cfg(test)]
mod tests {
    fn empty_trigram_file() {
        let ass = Array3::<u16>::zeros((26,26,26));
        println!("{}", serde_json::to_string(&ass).unwrap());
    }

    use super::*;
    #[test]
    fn it_works() {
        // empty_trigram_file();
    }
}

