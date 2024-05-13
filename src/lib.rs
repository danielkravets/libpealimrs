pub mod word_index;
pub mod word_dto;
mod prefix_tree;
mod util;
mod proto;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::word_index::WordIndex;

    #[test]
    fn load_and_build_index_from_pb() {
        let start = Instant::now();
        let index = WordIndex::init_local();
        let end = Instant::now();
        println!("Index build in: {:?}ms", end.duration_since(start));
        assert_eq!(index.get("לָלֶכֶת")[0].word_normalized, "ללכת");
        // assert_ne!(index.matching_forms())
        // assert_eq!(index.get("לפגוע")[0].word_normalized, "לפגוע");
        // assert_eq!(index.get("חוֹלוֵשׁ")[0].word_normalized, "לחלוש");
        // assert_eq!(index.get("לִפְגֹּועַ")[0].word_normalized, "לפגוע");
    }
}
