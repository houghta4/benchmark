mod trie;
mod utils;

use crate::utils::*;



fn main() {
    let v = vec_init();
    println!("v.len(): {}", v.len());
    println!("random word: {}", v.vec_lookup());

    let t = trie_init();
    println!("t.len(): {}", t.len());
    println!("random word: {}", t.get_random_word().unwrap());

    let h = hash_init();
    println!("h.len(): {}", h.len());
    println!("random word: {}", h.hash_lookup());
}
