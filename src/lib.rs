use std::io;
use rmp_serde::decode::Error;

use rmp_serde::from_read;

#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::word_dto::WordData;

mod word_dto;
mod word_index;
mod util;
mod prefix_tree;

const WORDS_MSP: &'static [u8] = include_bytes!("../words/words.msp");


pub fn load_data(data: &[u8]) -> Result<Vec<WordData>, Error> {
    let my_data: Vec<WordData> = from_read(&data[..])?;

    Ok(my_data)
}

#[cfg(not(feature = "wasm-support"))]
pub fn init(data: &[u8]) -> word_index::WordIndex {
    let words = load_data(data).unwrap();
    word_index::WordIndex::build(words)
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
pub fn init_local() -> word_index::WordIndex {
    let words = load_data(WORDS_MSP).unwrap();
    word_index::WordIndex::build(words)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn load_words() {
    //     let json = read_file("words/words.json").unwrap();
    //     let result = load_data(json.as_str());
    //     match result {
    //         Ok(words) => {
    //             assert_ne!(words.len(), 4);
    //         }
    //         Err(e) => {
    //             println!("Error: {:?}", e);
    //             assert!(false);
    //         }
    //     }
    // }

    use std::time::Instant;
    use crate::{init, WORDS_MSP};

    #[test]
    fn load_and_build_index() {
        let start = Instant::now();
        let index = init(WORDS_MSP);
        let end = Instant::now();
        println!("Index build in: {:?}ms", end.duration_since(start));
        assert_eq!(index.get("לָלֶכֶת")[0].word_normalized, "ללכת");
        // assert_eq!(index.get("לפגוע")[0].word_normalized, "לפגוע");
        // assert_eq!(index.get("חוֹלוֵשׁ")[0].word_normalized, "לחלוש");
        // assert_eq!(index.get("לִפְגֹּועַ")[0].word_normalized, "לפגוע");
    }
}
