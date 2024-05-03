use std::collections::{HashMap, HashSet};
use crate::util::normalize;
use crate::word_dto::WordData;
#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;
use crate::prefix_tree::Trie;


#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
pub struct WordIndex {
    data: HashMap<String, WordData>,
    index: HashMap<String, HashSet<String>>,
    prefix_tree: Trie,
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
impl WordIndex {
    pub fn build(words: Vec<WordData>) -> WordIndex {

        // collect words vector into a hashmap with url_id as key
        let data_index: HashMap<String, WordData> = words.iter().map(|word| (word.url_id.clone(), word.clone())).collect();

        let mut trie = Trie::new();

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
        // println!("Index size: {}", inf_index.len());
        WordIndex { data: data_index, index: inf_index, prefix_tree: trie}
    }

    pub fn get(&self, word: &str) -> Vec<WordData> {
        let word_norm = normalize(word);
        let val = self.index.get(word_norm.as_str());
        match val {
            Some(v) => {
                let url_ids: Vec<String> = v.iter().cloned().collect();
                url_ids.iter().map(|id| self.data.get(id).unwrap().clone()).collect()
            }
            None => vec![],
        }
    }

    pub fn suggest(&self, prefix: &str, limit: usize) -> Vec<String> {
        self.prefix_tree.find(prefix, limit)
    }
}