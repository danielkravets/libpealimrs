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
impl WordForm {
    #[cfg_attr(feature = "wasm-support", wasm_bindgen(constructor))]
    pub fn new(
        tense: String,
        person: String,
        number: String,
        gender: String,
        form: String,
        form_normalized: String,
        transcription: String,
        meaning: String,
        form_vowelled: Option<String>,
    ) -> WordForm {
        WordForm {
            tense,
            person,
            number,
            gender,
            form,
            form_normalized,
            transcription,
            meaning,
            form_vowelled,
        }
    }
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen(getter_with_clone))]
#[derive(Serialize, Deserialize, Clone)]
pub struct WordData {
    pub url_id: String,
    pub word: String,
    pub word_en: String,
    pub word_normalized: String,
    pub transcription: String,
    pub root: String,
    pub forms: Vec<WordForm>,
    pub binyan: String,
    pub passive: Option<Vec<WordForm>>,
    pub passive_binyan: Option<String>,
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen(getter_with_clone))]
impl WordData {
    #[cfg_attr(feature = "wasm-support", wasm_bindgen(constructor))]
    pub fn new(
        url_id: String,
        word: String,
        word_en: String,
        word_normalized: String,
        transcription: String,
        root: String,
        forms: Vec<WordForm>,
        binyan: String,
        passive: Option<Vec<WordForm>>,
        passive_binyan: Option<String>,
    ) -> WordData {
        WordData {
            url_id,
            word,
            word_en,
            word_normalized,
            transcription,
            root,
            forms,
            binyan,
            passive,
            passive_binyan,
        }
    }
}