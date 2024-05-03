extern crate unicode_normalization;

use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

pub(crate) fn normalize(val: &str) -> String {
    val.nfkd()  // Normalize using NFKD
        .filter(|c| !is_combining_mark(c.clone()))  // Filter out combining marks
        .collect::<String>()
}