use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use rand::{thread_rng, seq::SliceRandom, Rng};

use crate::trie::Trie;

pub fn vec_init() -> Vec<String> {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let mut word_list: Vec<String> = Vec::with_capacity(35);

    for line in reader.lines() {
        if let Ok(words) = line {
            word_list.push(words.trim().to_string());
        }
    }
    word_list
}
pub trait VecLookup {
    fn vec_lookup(&self) -> String;
}
impl VecLookup for Vec<String> {
    fn vec_lookup(&self) -> String {
        let mut rng = thread_rng();
        let word = self.choose(&mut rng).unwrap().to_string();
        word
    }
}

pub fn hash_init() -> HashMap<usize, String> {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let mut word_list: HashMap<usize, String> = HashMap::new();
    for (k, line) in reader.lines().enumerate() {
        if let Ok(words) = line {
            word_list.insert(k, words);
        }
    }
    word_list
}

pub trait HashLookup {
    fn hash_lookup(&self) -> String;
}
impl HashLookup for HashMap<usize, String> {
    fn hash_lookup(&self) -> String {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..self.len());
        let word = self.get(&idx).unwrap().to_string();
        word
    }
}

pub fn trie_init() -> Trie {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let mut word_list = Trie::new();

    for line in reader.lines() {
        if let Ok(words) = line {
            word_list.insert(&words);
        }
    }
    word_list
}

