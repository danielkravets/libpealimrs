use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    id: Option<String>,
    is_word_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            id: None,
            is_word_end: false,
        }
    }
}


pub(crate) struct Trie {
    root: TrieNode,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie { root: TrieNode::new() }
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
            node = node.children.entry(c).or_insert_with(TrieNode::new)
        }
        node.is_word_end = true;
        node.id = Some(id);
    }

    pub(crate) fn find_string(&self, prefix: String) -> Vec<String> {
        return self.find(&prefix);
    }

    pub(crate) fn find(&self, prefix: &str) -> Vec<String> {
        // let mut results = Vec::new();
        let node_opt = self.starts_with(prefix);

        return match node_opt {
            None => Vec::new(),
            Some(n) => self.get_all_ids_from(n),
        };
    }

    fn get_all_ids_from(&self, node: &TrieNode) -> Vec<String> {
        let mut ids = Vec::new();
        let mut stack = VecDeque::new();
        stack.push_back(node);
        while let Some(node) = stack.pop_front() {
            if node.is_word_end {
                ids.push(node.id.clone().unwrap());
            }
            for (_, child_node) in &node.children {
                stack.push_back(child_node);
            }
        }
        return ids;
    }

    fn starts_with(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return None,
            }
        }
        return Some(node);
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
        trie.insert(String::from("lincoln"), String::from("2"));

        let vec = trie.find("lab");
        // assert both "1" and "2" are in the vec
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.contains(&String::from("1")), true);
        assert_eq!(vec.contains(&String::from("2")), true);
    }

    #[test]
    fn test_table() {
        let test_cases = vec![
            TestCase {
                words_to_insert: vec!["hello", "hell", "helium"],
                searches: vec![("hello", 1), ("hell", 2), ("helium", 1), ("help", 0), ("he", 3)],
            },
            TestCase {
                words_to_insert: vec!["apple", "app"],
                searches: vec![("app", 2), ("apple", 1), ("application", 0)],
            },
            TestCase {
                words_to_insert: vec!["rust", "rusty", "rustic"],
                searches: vec![("rust", 3), ("rusty", 1), ("rustic", 1), ("rustling", 0)],
            },
        ];

        for case in test_cases {
            let mut trie = Trie::new();
            for word in case.words_to_insert {
                trie.insert(String::from(word), String::from(word));
            }
            for (search_word, expected) in case.searches {
                assert_eq!(trie.find(search_word).len(), expected, "Failed search for '{}'", search_word);
            }
        }
    }
}