use std::time::Instant;
use libpealimrs::word_index;
use libpealimrs::word_dto;

fn main() {
    let start = Instant::now();
    for libpealimrs in 0..2 {
        let index = word_index::WordIndex::init_local();
    }
    let end = Instant::now();
    println!("Index build in: {:?}ms", end.duration_since(start));
}
