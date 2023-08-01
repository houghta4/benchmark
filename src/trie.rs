use std::collections::HashMap;
use rand::seq::IteratorRandom;
use rand::thread_rng;

// TrieNode represents a node in the trie
#[derive(Default, Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end_of_word: bool,
}

// Trie represents the trie data structure
#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,
    pub size: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // Insert a word into the trie
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
        self.size += 1;
    }

    // Get a random word from the trie
    pub fn get_random_word(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.get_random_word_from_node(&self.root, &mut rng)
    }

    fn get_random_word_from_node<R: rand::Rng>(
        &self,
        node: &TrieNode,
        rng: &mut R,
    ) -> Option<String> {
        let mut current_node = node;
        let mut current_word = String::new();

        loop {
            let candidates: Vec<_> = current_node.children.keys().collect();
            if candidates.is_empty() {
                // If there are no child nodes, return None
                return None;
            }

            // Choose a random character from the candidates
            let random_char = *candidates.iter().choose(rng).unwrap();
            current_word.push(*random_char);

            // Get the child node corresponding to the random character
            if let Some(child_node) = current_node.children.get(&random_char) {
                current_node = child_node;

                // If the current node marks the end of a word, return the current word
                if current_node.is_end_of_word {
                    return Some(current_word);
                }
            } else {
                // The random character does not correspond to a child node, return None
                return None;
            }
        }
    }
}
