use std::collections::{HashMap, HashSet};

use prost::Message;
use rmp_serde::decode::Error;
use rmp_serde::from_read;
#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::prefix_tree::Trie;
use crate::proto::{convert_pb_to_dto, WORDS_PB};
use crate::util::normalize;
use crate::word_dto::{WordData, WordForm};

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
pub struct WordIndex {
    data: HashMap<String, WordData>,
    index: HashMap<String, HashSet<String>>,
    roots_index: HashMap<String, HashSet<String>>,
    prefix_tree: Trie,
}

impl WordIndex {
    pub fn load_data(data: &[u8]) -> Result<Vec<WordData>, Error> {
        let my_data: Vec<WordData> = from_read(&data[..])?;
        Ok(my_data)
    }
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
impl WordIndex {

    pub fn init_local() -> WordIndex {
        let word_list: crate::proto::worddata::WordDataList = Message::decode(WORDS_PB).unwrap();
        let word_datas: Vec<WordData> = convert_pb_to_dto(word_list.words);
        let index = WordIndex::build(word_datas);
        index
    }

    pub fn build(words: Vec<WordData>) -> WordIndex {

        // collect words vector into a hashmap with url_id as key
        let data_index: HashMap<String, WordData> = words.iter().map(|word| (word.url_id.clone(), word.clone())).collect();

        let mut trie = Trie::new();

        let roots_index: HashMap<String, HashSet<String>> = words.iter().map(|word| (
            word.root.clone(), HashSet::from([word.url_id.clone()])
        )).collect();

        let mut inf_index: HashMap<String, HashSet<String>> = words.iter().map(|word| (
            word.word_normalized.clone(), HashSet::from([word.url_id.clone()])  //vec! {word.url_id.clone()}
        )).collect();
        for word in &words {
            for form in &word.forms {
                if form.form_normalized.is_empty() {
                    continue;
                }
                if inf_index.contains_key(&form.form_normalized) {
                    // println!("Duplicate form: {}", form.form_normalized);
                    inf_index.get_mut(&form.form_normalized).unwrap().insert(word.url_id.clone());
                } else {
                    inf_index.insert(form.form_normalized.clone(), HashSet::from([word.url_id.clone()]));
                }
            }
        }

        for (form, data) in &inf_index {
            for url_id in data {
                trie.insert(form.clone(), url_id.clone());
            }
        }
        WordIndex {
            data: data_index,
            index: inf_index,
            prefix_tree: trie,
            roots_index: roots_index,
        }
    }

    fn collect_word_data_by_ids(&self, ids: &HashSet<String>) -> Vec<WordData> {
        ids.iter().map(|id| self.data.get(id).unwrap().clone()).collect()
    }

    pub fn get_by_root(&self, root: &str) -> Vec<WordData> {
        let val = self.roots_index.get(root);
        match val {
            Some(v) => {
                self.collect_word_data_by_ids(v)
            }
            None => vec![],
        }
    }

    pub fn get(&self, word: &str) -> Vec<WordData> {
        let word_norm = normalize(word);
        let val = self.index.get(word_norm.as_str());
        match val {
            Some(v) => {
                self.collect_word_data_by_ids(v)
            }
            None => vec![],
        }
    }

    pub fn suggest(&self, prefix: &str, limit: usize) -> Vec<WordData> {
        let ids = self.prefix_tree.find(prefix, limit);
        ids.iter().map(|id| self.data.get(id).unwrap().clone()).collect()
    }


    pub fn matching_forms(&self, word_id: &str, form_str: &str) -> Vec<WordForm> {
        let wd = self.data.get(word_id);
        return match wd {
            None => {
                vec![]
            }
            Some(word_data) => {
                let mut matches: Vec<WordForm> = Vec::new();
                for form in &word_data.forms {
                    if form.form_normalized == form_str {
                        matches.push(form.clone());
                    }
                }
                matches
            }
        };
    }
}