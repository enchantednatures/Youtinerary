#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn new(children: HashMap<char, TrieNode>, word: Option<String>) -> Self {
        Self { children, word }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
    fuzzy: bool,
}
impl Default for Trie {
    fn default() -> Self {
        Self {
            root: TrieNode::new(HashMap::new(), None),
            fuzzy: true,
        }
    }
}

impl Trie {
    fn new(root: TrieNode, fuzzy: bool) -> Self {
        Self { root, fuzzy }
    }

    fn insert(&mut self, mut word: String) {
        if word.is_empty() {
            return;
        }

        word = word.to_lowercase();
        let mut current_node = &mut self.root;
        for c in word.chars() {
            if current_node.children.get(&c).is_none() {
                current_node.children.insert(c, TrieNode::default());
            }
            current_node = current_node.children.get_mut(&c).unwrap();
        }

        current_node.word = Some(word);
    }

    fn search(&self, word: &str, limit: Option<u8>) -> Vec<String> {
        let mut current_node = &self.root;
        let mut result = Vec::new();
        let mut word = word.to_string();

        word = word.to_lowercase();
        for c in word.chars() {
            if current_node.children.get(&c).is_none() {
                return result;
            }
            current_node = current_node.children.get(&c).unwrap();
        }
        // if let Some(limit) = limit {
        //     self.search_recursive(&current_node, &mut result, limit);
        // } else {
        //     self.search_recursive(&current_node, &mut result, 0);
        // }
        result
    }
}
impl TrieNode {
    fn collect_all_descendent_words(&self) -> Vec<&str> {
        let mut collection = Vec::new();
        self.children.iter().for_each(|(_, c)| {
            if let Some(word) = &c.word {
                collection.push(word.as_str());
            }
            collection.extend(c.collect_all_descendent_words());
        });
        collection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_insertion() {
        let mut tree = Trie::default();
        tree.insert("hello".to_string());
        tree.insert("world".to_string());
        tree.insert("hello world".to_string());
        tree.insert("helloworld".to_string());
        tree.insert("Goodbye, Mars".to_string());
        println!("{:#?}", tree);
        let root = tree.root;
        let desc = root.collect_all_descendent_words();

        println!("{:#?}", desc);
        // dbg!(&desc);
        assert!(desc.contains(&"hello"));
    }
}
