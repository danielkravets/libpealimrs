use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    ids: Option<HashSet<String>>,
    is_word_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            ids: None,
            is_word_end: false,
        }
    }
}

pub(crate) struct Trie {
    root: TrieNode,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub(crate) fn insert(&mut self, word: String, id: String) {
        let mut node = &mut self.root;
        /*
        TODO: From rust Docs
        It's important to remember that char represents a Unicode Scalar Value,
        and might not match your idea of what a 'character' is.
        Iteration over grapheme clusters may be what you actually want.
        This functionality is not provided by Rust's standard library, check crates. io instead.
        */
        for c in word.chars() {
            node = node.children.entry(c).or_default()
        }
        // if node.is_word_end && node.ids != None {
        //     println!("Duplicate word: {}", word);
        // }
        node.is_word_end = true;
        if node.ids.is_none() {
            node.ids = Some(HashSet::new());
        }
        node.ids.as_mut().unwrap().insert(id);
    }

    pub(crate) fn find_string(&self, prefix: String, limit: usize) -> Vec<String> {
        self.find(&prefix, limit)
    }

    pub(crate) fn find(&self, prefix: &str, limit: usize) -> Vec<String> {
        // let mut results = Vec::new();
        let node_opt = self.starts_with(prefix);

        match node_opt {
            None => Vec::new(),
            Some(n) => self.get_all_ids_from(n, limit),
        }
    }

    fn get_all_ids_from(&self, node: &TrieNode, limit: usize) -> Vec<String> {
        let mut found_ids = HashSet::new();
        let mut ids = Vec::new();
        let mut stack = VecDeque::new();
        stack.push_back(node);
        while let Some(node) = stack.pop_front() {
            if ids.len() >= limit {
                break;
            }
            if node.is_word_end {
                for id in node.ids.as_ref().unwrap() {
                    if !found_ids.contains(id) {
                        ids.push(id.clone());
                        found_ids.insert(id.clone());
                    }
                }
            }
            for child_node in node.children.values() {
                stack.push_back(child_node);
            }
        }
        // limit ids to limit
        ids.truncate(limit);
        ids
    }

    fn starts_with(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return None,
            }
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::prefix_tree::Trie;
    struct TestCase {
        words_to_insert: Vec<&'static str>,
        searches: Vec<(&'static str, usize)>,
    }

    #[test]
    fn test_trie_basic() {
        let mut trie = Trie::new();

        trie.insert(String::from("laboratory"), String::from("1"));
        trie.insert(String::from("labrador"), String::from("2"));
        trie.insert(String::from("lincoln"), String::from("3"));

        let vec = trie.find("lab", 15);
        // assert both "1" and "2" are in the vec
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&String::from("1")));
        assert!(vec.contains(&String::from("2")));
    }

    #[test]
    fn test_trie_limit() {
        let mut trie = Trie::new();

        trie.insert(String::from("laboratory"), String::from("1"));
        trie.insert(String::from("labrador"), String::from("2"));
        trie.insert(String::from("labyrinth"), String::from("3"));

        let vec = trie.find("lab", 2);
        // assert both "1" and "2" are in the vec
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&String::from("2")));
        assert!(vec.contains(&String::from("3")));
    }

    #[test]
    fn test_table() {
        let test_cases = vec![
            TestCase {
                words_to_insert: vec!["hello", "hell", "helium"],
                searches: vec![
                    ("hello", 1),
                    ("hell", 2),
                    ("helium", 1),
                    ("help", 0),
                    ("he", 3),
                ],
            },
            TestCase {
                words_to_insert: vec!["apple", "app"],
                searches: vec![("app", 2), ("apple", 1), ("application", 0)],
            },
            TestCase {
                words_to_insert: vec!["rust", "rusty", "rustic"],
                searches: vec![("rust", 3), ("rusty", 1), ("rustic", 1), ("rustling", 0)],
            },
            TestCase {
                words_to_insert: vec!["apple", "apple"],
                searches: vec![("apple", 2)],
            },
        ];

        for case in test_cases {
            let mut trie = Trie::new();
            for (i, &word) in case.words_to_insert.iter().enumerate() {
                trie.insert(String::from(word), i.to_string());
            }
            for (search_word, expected) in case.searches {
                let results = trie.find(search_word, 15);
                assert_eq!(
                    results.len(),
                    expected,
                    "Failed search for '{}'",
                    search_word
                );
            }
        }
    }
}
