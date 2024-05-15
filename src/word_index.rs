use std::collections::{HashMap, HashSet};

use prost::Message;
use rmp_serde::decode::Error;
use rmp_serde::from_read;
#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::prefix_tree::Trie;
use crate::proto::{convert_pb_to_dto, WORDS_PB};
use crate::util::normalize;
use crate::word_dto::{SearchResult, WordData};

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

        let mut hebrew_index: HashMap<String, HashSet<String>> = HashMap::new();
        for word in &words {
            if hebrew_index.contains_key(&word.word_normalized) {
                // println!("Duplicate form: {}", word.word_normalized);
                hebrew_index.get_mut(&word.word_normalized).unwrap().insert(word.url_id.clone());
            } else {
                hebrew_index.insert(word.word_normalized.clone(), HashSet::from([word.url_id.clone()]));
            }
        }
        for word in &words {
            for form in &word.forms {
                if form.form_normalized.is_empty() {
                    continue;
                }
                if hebrew_index.contains_key(&form.form_normalized) {
                    // println!("Duplicate form: {}", form.form_normalized);
                    hebrew_index.get_mut(&form.form_normalized).unwrap().insert(word.url_id.clone());
                } else {
                    hebrew_index.insert(form.form_normalized.clone(), HashSet::from([word.url_id.clone()]));
                }
            }
        }

        for (form, data) in &hebrew_index {
            for url_id in data {
                trie.insert(form.clone(), url_id.clone());
            }
        }
        WordIndex {
            data: data_index,
            index: hebrew_index,
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

    pub fn get(&self, word: &str) -> Vec<SearchResult> {
        let word_norm = normalize(word);
        let val = self.index.get(word_norm.as_str());
        match val {
            Some(v) => {
                self.collect_word_data_by_ids(v).iter().map(|wd| SearchResult {
                    word: wd.clone(),
                    matching_forms: WordIndex::matching_forms_inner(wd, word_norm.as_str()),
                }).collect()
            }
            None => vec![],
        }
    }

    pub fn suggest(&self, prefix: &str, limit: usize) -> Vec<SearchResult> {
        let prefix_norm = normalize(prefix);
        let ids = self.prefix_tree.find(prefix_norm.as_str(), limit);
        let word_datas: Vec<WordData> = ids.iter().map(|id| self.data.get(id).unwrap().clone()).collect();
        word_datas.iter().map(|wd| SearchResult {
            word: wd.clone(),
            matching_forms: WordIndex::matching_forms_inner(wd, prefix_norm.as_str()),
        }).collect()
    }


    pub fn matching_forms(&self, word_id: &str, form_str: &str) -> Vec<usize> {
        let form_str_norm = normalize(form_str);
        let wd = self.data.get(word_id);
        return match wd {
            None => {
                vec![]
            }
            Some(word_data) => {
                WordIndex::matching_forms_inner(word_data, form_str_norm.as_str())
            }
        };
    }

    fn matching_forms_inner(word_data: &WordData, form_str_norm: &str) -> Vec<usize> {
        let mut matches: Vec<usize> = Vec::new();
        if word_data.word_normalized == form_str_norm {
            // infinitive form is not in the list of forms
            // usize can't be negative, so we will use word_data.forms.len() as an indication
            // that it matches the infinitive form
            matches.push(word_data.forms.len());
        }
        for (i, form) in word_data.forms.iter().enumerate() {
            if form.form_normalized == form_str_norm {
                matches.push(i);
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use crate::word_index::WordIndex;

    #[test]
    fn load_and_build_index_from_pb() {
        let index = WordIndex::init_local();
        let vec = index.matching_forms("9-lashevet", "תשב");
        println!("results: {:?}", vec.len());
        assert_eq!(vec.len(), 2);
    }
}