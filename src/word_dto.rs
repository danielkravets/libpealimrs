use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm-support", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Clone)]
pub struct WordForm {
    pub tense: String,
    pub person: String,
    pub number: String,
    pub gender: String,
    pub form: String,
    pub form_normalized: String,
    pub transcription: String,
    pub meaning: String,
    pub form_vowelled: Option<String>,
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Clone)]
pub struct WordData {
    pub url_id: String,
    pub word: String,
    pub word_en: String,
    pub word_normalized: String,
    pub transcription: String,
    pub root: Vec<String>,
    pub forms: Vec<WordForm>,
    pub binyan: String,
    pub passive: Option<Vec<WordForm>>,
    pub passive_binyan: Option<String>,
}