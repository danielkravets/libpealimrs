use std::collections::HashMap;
use std::io::Read;

use lazy_static::lazy_static;
use prost;
use prost::Message;

use crate::proto::worddata::{Binyan, Gender, Number, Person, Tense, WordData as WordDataPB, WordDataList, WordForm as WordFormPB};
use crate::word_dto::{WordData, WordForm};
pub(crate) const WORDS_PB: &'static [u8] = include_bytes!("../words/words.pb");
mod build;
pub(crate) mod worddata;
lazy_static! {
    static ref BINYANS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("paal", "PA'AL");
        m.insert("nifal", "NIF'AL");
        m.insert("piel", "PI'EL");
        m.insert("pual", "PU'AL");
        m.insert("hifil", "HIF'IL");
        m.insert("hufal", "HUF'AL");
        m.insert("hitpael", "HITPA'EL");
        m
    };
}

pub(crate) fn read_from_bytes() -> Vec<WordDataPB> {
    const WORDS_PB: &'static [u8] = include_bytes!("../words/words.pb");
    let word_list: WordDataList = prost::Message::decode(WORDS_PB).unwrap();
    return word_list.words;
}

pub(crate) fn convert_pb_to_dto(src: Vec<WordDataPB>) -> Vec<WordData> {
    src.iter().map(|word_data_pb| convert_word_data_pb_to_dto(word_data_pb.clone())).collect()
}

fn convert_word_data_pb_to_dto(word_data_pb: WordDataPB) -> WordData {
    let passives = if word_data_pb.passive.is_empty() {
        None
    } else {
        Some(
            word_data_pb.passive.iter().map(|form| convert_word_form_pb_to_dto(form.clone())).collect()
        )
    };

    let passive_binyan = word_data_pb.passive_binyan.map(|binyan| convert_binyan_pb_to_dto(binyan));
    WordData {
        url_id: word_data_pb.url_id,
        word: word_data_pb.word,
        word_en: word_data_pb.word_en,
        word_normalized: word_data_pb.word_normalized,
        transcription: word_data_pb.transcription,
        root: word_data_pb.root,
        forms: word_data_pb.forms.iter().map(|form| convert_word_form_pb_to_dto(form.clone())).collect(),
        binyan: convert_binyan_pb_to_dto(word_data_pb.binyan),
        passive: passives,
        passive_binyan: passive_binyan,
    }
}

fn convert_word_form_pb_to_dto(word_form_pb: WordFormPB) -> WordForm {
    WordForm {
        tense: convert_tense_pb_to_dto(word_form_pb.tense),
        person: convert_person_pb_to_dto(word_form_pb.person),
        number: convert_number_pb_to_dto(word_form_pb.number),
        gender: convert_gender_pb_to_dto(word_form_pb.gender),
        form: word_form_pb.form,
        form_normalized: word_form_pb.form_normalized,
        transcription: word_form_pb.transcription,
        meaning: word_form_pb.meaning,
        form_vowelled: word_form_pb.form_vowelled,
    }
}

fn convert_tense_pb_to_dto(tense_pb: i32) -> String {
    match tense_pb {
        0 => Tense::Past,
        1 => Tense::Present,
        2 => Tense::Future,
        _ => panic!("Invalid tense value: {}", tense_pb),
    }.as_str_name().to_lowercase()
}

fn convert_binyan_pb_to_dto(binyan_pb: i32) -> String {
    let binyan_raw = match binyan_pb {
        0 => Binyan::Paal,
        1 => Binyan::Nifal,
        2 => Binyan::Piel,
        3 => Binyan::Pual,
        4 => Binyan::Hifil,
        5 => Binyan::Hufal,
        6 => Binyan::Hitpael,
        _ => panic!("Invalid binyan value: {}", binyan_pb),
    }.as_str_name().to_lowercase();
    return BINYANS.get(binyan_raw.as_str()).unwrap().to_string();
}

fn convert_gender_pb_to_dto(gender: i32) -> String {
    let val = match gender {
        0 => Gender::M,
        1 => Gender::F,
        2 => Gender::AllG,
        _ => panic!("Invalid binyan value: {}", gender),
    }.as_str_name().to_lowercase();
    return unify_all(val);
}

fn convert_number_pb_to_dto(number_pb: i32) -> String {
    match number_pb {
        0 => Number::Singular,
        1 => Number::Plural,
        _ => panic!("Invalid number value: {}", number_pb),
    }.as_str_name().to_lowercase()
}

fn convert_person_pb_to_dto(person_pb: i32) -> String {
    let val = match person_pb {
        0 => Person::P1st,
        1 => Person::P2nd,
        2 => Person::P3rd,
        3 => Person::AllP,
        _ => panic!("Invalid person value: {}", person_pb),
    }.as_str_name().to_lowercase();
    return unify_all(val);
}

fn unify_all(val: String) -> String {
    if val.starts_with("all") {
        return "all".to_string();
    }
    return val;
}


#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::proto::{convert_pb_to_dto, WORDS_PB};
    use crate::proto::worddata::WordDataList;
    use crate::word_dto::WordData;

    #[test]
    fn load_proto() {
        let word_list: WordDataList = Message::decode(WORDS_PB).unwrap();
        let word_datas: Vec<WordData> = convert_pb_to_dto(word_list.words);
        println!("word_list: {:?}", word_datas.len());
    }
}
