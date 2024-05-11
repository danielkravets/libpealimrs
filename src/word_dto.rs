use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm-support")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
#[derive(Serialize, Deserialize, Clone)]
pub struct WordForm {
    pub(crate) tense: String,
    pub(crate) person: String,
    pub(crate) number: String,
    pub(crate) gender: String,
    pub(crate) form: String,
    pub(crate) form_normalized: String,
    pub(crate) transcription: String,
    pub(crate) meaning: String,
    pub(crate) form_vowelled: Option<String>,
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
#[derive(Serialize, Deserialize, Clone)]
pub struct WordData {
    pub(crate) url_id: String,
    pub(crate) word: String,
    pub(crate) word_en: String,
    pub(crate) word_normalized: String,
    pub(crate) transcription: String,
    pub(crate) root: Vec<String>,
    pub(crate) forms: Vec<WordForm>,
    pub(crate) binyan: String,
    pub(crate) passive: Option<Vec<WordForm>>,
    pub(crate) passive_binyan: Option<String>,
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
impl WordForm {
    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn tense(&self) -> String {
        self.tense.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn person(&self) -> String {
        self.person.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn number(&self) -> String {
        self.number.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn gender(&self) -> String {
        self.gender.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn form(&self) -> String {
        self.form.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn form_normalized(&self) -> String {
        self.form_normalized.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn transcription(&self) -> String {
        self.transcription.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn meaning(&self) -> String {
        self.meaning.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn form_vowelled(&self) -> String {
        if let Some(v) = &self.form_vowelled {
            v.clone() // Explicitly clone the String
        } else {
            "".to_string()
        }
    }
}

#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
impl WordData {
    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn url_id(&self) -> String {
        self.url_id.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn word(&self) -> String {
        self.word.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn word_en(&self) -> String {
        self.word_en.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn word_normalized(&self) -> String {
        self.word_normalized.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn transcription(&self) -> String {
        self.transcription.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn root(&self) -> Vec<String> {
        self.root.clone() // Explicitly clone the Vec
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn forms(&self) -> Vec<WordForm> {
        self.forms.clone() // Explicitly clone the Vec
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn binyan(&self) -> String {
        self.binyan.clone() // Explicitly clone the String
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn passive(&self) -> Option<Vec<WordForm>> {
        self.passive.clone() // Explicitly clone the Option
    }

    #[cfg_attr(feature = "wasm-support", wasm_bindgen(getter))]
    pub fn passive_binyan(&self) -> Option<String> {
        self.passive_binyan.clone() // Explicitly clone the Option
    }

    /*#[cfg_attr(feature = "wasm-support", wasm_bindgen)]
    pub fn matching_forms(&self, form_str: &str) -> Vec<WordForm> {
        let mut matches: Vec<WordForm> = Vec::new();
        for form in &self.forms {
            if form.form_normalized == form_str {
                matches.push(form.clone());
            }
        }
        return matches;
    }*/
}

// implementation of parsing from json into struct
// #[wasm_bindgen]
// impl WordData {
//     pub fn from_json(json: &str) -> Result<WordData, serde_json::Error> {
//         let val = serde_json::from_str(json);
//         match val {
//             Ok(v) => Ok(v),
//             Err(e) => WordData,
//         }
//     }
// }