use std::collections::{HashMap, HashSet};

use prost::Message;
use regex::Regex;
use rmp_serde::decode::Error;
use rmp_serde::from_read;
#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::prefix_tree::Trie;
use crate::proto::{convert_pb_to_dto, WORDS_PB};
use crate::util::normalize;
use crate::word_dto::{MatchedForm, SearchResult, WordData};

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
pub struct WordIndex {
    data: HashMap<String, WordData>,
    index: HashMap<String, HashSet<String>>,
    roots_index: HashMap<String, HashSet<String>>,
    prefix_tree: Trie,
    prefix_tree_en: Trie,
}

impl WordIndex {
    pub fn load_data(data: &[u8]) -> Result<Vec<WordData>, Error> {
        let my_data: Vec<WordData> = from_read(data)?;
        Ok(my_data)
    }
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
impl WordIndex {
    pub fn init_local() -> WordIndex {
        let word_list: crate::proto::worddata::WordDataList = Message::decode(WORDS_PB).unwrap();
        let word_datas: Vec<WordData> = convert_pb_to_dto(word_list.words);

        WordIndex::build(word_datas)
    }

    pub fn build(words: Vec<WordData>) -> WordIndex {
        // collect words vector into a hashmap with url_id as key
        let data_index: HashMap<String, WordData> = words
            .iter()
            .map(|word| (word.url_id.clone(), word.clone()))
            .collect();

        let mut trie = Trie::new();

        let roots_index: HashMap<String, HashSet<String>> = words
            .iter()
            .map(|word| (word.root.clone(), HashSet::from([word.url_id.clone()])))
            .collect();

        let mut translation_index = Trie::new();
        for word in &words {
            let translations = WordIndex::get_translations(word);
            for translation in translations {
                translation_index.insert(translation, word.url_id.clone());
            }
        }

        let mut hebrew_index: HashMap<String, HashSet<String>> = HashMap::new();
        for word in &words {
            if hebrew_index.contains_key(&word.word_normalized) {
                // println!("Duplicate form: {}", word.word_normalized);
                hebrew_index
                    .get_mut(&word.word_normalized)
                    .unwrap()
                    .insert(word.url_id.clone());
            } else {
                hebrew_index.insert(
                    word.word_normalized.clone(),
                    HashSet::from([word.url_id.clone()]),
                );
            }
        }
        for word in &words {
            for form in &word.forms {
                if form.form_normalized.is_empty() {
                    continue;
                }
                if hebrew_index.contains_key(&form.form_normalized) {
                    // println!("Duplicate form: {}", form.form_normalized);
                    hebrew_index
                        .get_mut(&form.form_normalized)
                        .unwrap()
                        .insert(word.url_id.clone());
                } else {
                    hebrew_index.insert(
                        form.form_normalized.clone(),
                        HashSet::from([word.url_id.clone()]),
                    );
                }
            }
            match &word.passive {
                None => {}
                Some(passive) => {
                    for form in passive {
                        if form.form_normalized.is_empty() {
                            continue;
                        }
                        if hebrew_index.contains_key(&form.form_normalized) {
                            // println!("Duplicate form: {}", form.form_normalized);
                            hebrew_index
                                .get_mut(&form.form_normalized)
                                .unwrap()
                                .insert(word.url_id.clone());
                        } else {
                            hebrew_index.insert(
                                form.form_normalized.clone(),
                                HashSet::from([word.url_id.clone()]),
                            );
                        }
                    }
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
            prefix_tree_en: translation_index,
            roots_index,
        }
    }

    fn get_translations(word_data: &WordData) -> Vec<String> {
        // let word_en = word_data.word_en.clone();
        let no_braces = Self::remove_braces(word_data.word_en.as_str());
        let punkt_split: Vec<&str> = no_braces
            .split(|c: char| c.is_ascii_punctuation())
            .collect();
        // println!("punkt_split: {:?}", punkt_split);
        let result = punkt_split
            .iter()
            .map(|s| s.trim().trim_start_matches("to ").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        // println!("result: {:?}", result);
        result
    }

    fn remove_braces(input: &str) -> String {
        let re = Regex::new(r"\(.*?\)").unwrap();
        let result = re.replace_all(input, "");
        result.into_owned()
    }

    fn collect_word_data_by_ids(&self, ids: &HashSet<String>) -> Vec<WordData> {
        ids.iter()
            .map(|id| self.data.get(id).unwrap().clone())
            .collect()
    }

    pub fn get_by_root(&self, root: &str) -> Vec<WordData> {
        let val = self.roots_index.get(root);
        match val {
            Some(v) => self.collect_word_data_by_ids(v),
            None => vec![],
        }
    }

    pub fn get(&self, word: &str) -> Vec<SearchResult> {
        let word_norm = normalize(word);
        let val = self.index.get(word_norm.as_str());
        match val {
            Some(v) => self
                .collect_word_data_by_ids(v)
                .iter()
                .map(|wd| SearchResult {
                    word: wd.clone(),
                    matching_forms: WordIndex::matching_forms_inner(wd, word_norm.as_str()),
                })
                .collect(),
            None => vec![],
        }
    }

    pub fn suggest(&self, prefix: &str, limit: usize) -> Vec<SearchResult> {
        let prefix_norm = normalize(prefix).trim_start_matches("to ").to_string();
        let mut suggestions = self.suggest_hebrew(prefix_norm.as_str(), limit);
        if suggestions.is_empty() {
            suggestions = self.suggest_by_translation(prefix_norm.as_str(), limit);
        }
        suggestions
    }

    pub fn suggest_hebrew(&self, prefix_norm: &str, limit: usize) -> Vec<SearchResult> {
        // let prefix_norm = normalize(prefix);
        let ids = self.prefix_tree.find(prefix_norm, limit);
        let word_datas: Vec<WordData> = ids
            .iter()
            .map(|id| self.data.get(id).unwrap().clone())
            .collect();
        word_datas
            .iter()
            .map(|wd| SearchResult {
                word: wd.clone(),
                matching_forms: WordIndex::matching_forms_inner(wd, prefix_norm),
            })
            .collect()
    }

    pub fn suggest_by_translation(&self, prefix_norm: &str, limit: usize) -> Vec<SearchResult> {
        // let prefix_norm = normalize(prefix);
        let ids = self.prefix_tree_en.find(prefix_norm, limit);
        let word_datas: Vec<WordData> = ids
            .iter()
            .map(|id| self.data.get(id).unwrap().clone())
            .collect();
        word_datas
            .iter()
            .map(|wd| SearchResult {
                word: wd.clone(),
                matching_forms: WordIndex::matching_forms_inner(wd, prefix_norm),
            })
            .collect()
    }

    pub fn matching_forms(&self, word_id: &str, form_str: &str) -> Vec<MatchedForm> {
        let form_str_norm = normalize(form_str);
        let wd = self.data.get(word_id);
        match wd {
            None => {
                vec![]
            }
            Some(word_data) => WordIndex::matching_forms_inner(word_data, form_str_norm.as_str()),
        }
    }

    fn matching_forms_inner(word_data: &WordData, form_str_norm: &str) -> Vec<MatchedForm> {
        let mut matches: Vec<MatchedForm> = Vec::new();
        if word_data.word_normalized == form_str_norm {
            // infinitive form is not in the list of forms
            // usize can't be negative, so we will use word_data.forms.len() as an indication
            // that it matches the infinitive form
            matches.push(MatchedForm { index: 0, kind: 0 });
        }
        for (i, form) in word_data.forms.iter().enumerate() {
            if form.form_normalized == form_str_norm {
                matches.push(MatchedForm { index: i, kind: 1 });
            }
        }
        match &word_data.passive {
            None => {}
            Some(passive) => {
                for (i, form) in passive.iter().enumerate() {
                    if form.form_normalized == form_str_norm {
                        matches.push(MatchedForm { index: i, kind: 2 });
                    }
                }
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
    #[test]
    fn load_and_build_index_suggest() {
        let index = WordIndex::init_local();
        let vec = index.suggest("lea", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
    }
    #[test]
    fn load_and_build_index_suggest_passive() {
        let index = WordIndex::init_local();
        let vec = index.suggest("ללקט", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
    }
    #[test]
    fn load_and_build_index_suggest_matching_forms() {
        let index = WordIndex::init_local();
        let vec = index.suggest("ללח", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
    }
    #[test]
    fn load_and_build_index_suggest_matching_forms_pual() {
        let index = WordIndex::init_local();
        let vec = index.suggest("לְעוֹדֵד", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
        assert!(vec.len() == 1);
        assert_eq!(vec[0].word.passive_binyan.as_deref(), Some("PU'AL"));
    }
    #[test]
    fn load_and_build_index_suggest_matching_forms_hufal() {
        let index = WordIndex::init_local();
        let vec = index.suggest("להגזים", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
        assert!(vec.len() == 1);
        assert_eq!(vec[0].word.passive_binyan.as_deref(), Some("HUF'AL"));
    }
    #[test]
    fn load_and_build_index_suggest_matching_forms_pual_search() {
        let index = WordIndex::init_local();
        let vec = index.suggest("תנוסי", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
        // assert_eq!(vec.len() == 1, true);
        // assert_eq!(vec[0].word.passive_binyan.as_deref(), Some("PU'AL"));
    }
    #[test]
    fn load_and_build_index_suggest_matching_forms_hufal_search() {
        let index = WordIndex::init_local();
        let vec = index.suggest("תנוסינה", 15);
        println!("results: {:?}", vec.len());
        assert!(!vec.is_empty());
        // assert_eq!(vec.len() == 1, true);
        // assert_eq!(vec[0].word.passive_binyan.as_deref(), Some("HUF'AL"));
    }
}
