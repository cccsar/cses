//! Experimental implementation for a tree DS
//! Aim to progresively mutate implementation to incorporate Rust features

use std::ops::Deref;

static ALPHA_SIZE : usize = 27;
static DEFAULT : isize = -1;

/// Initial Implementation for a Trie Node providing two types of construction
#[derive(PartialEq, Eq, Debug)]
pub struct TrieNode {
    pub next_char : Vec<isize> ,
    pub is_terminal: bool
}

impl TrieNode {
    pub fn new(default : isize, alpha_size : usize) -> Self { 
        Self {
            next_char : vec![default; alpha_size],
            is_terminal : false,
        }
     }
    pub fn build(is_terminal : bool, default : isize, alpha_size : usize) -> Self { 
        Self {
            next_char : vec![default; alpha_size],
            is_terminal
        }
     }
}

/// Initial implementation for a tree providing construction and CRUD
#[derive(PartialEq, Eq, Debug)]
pub struct Trie {
    nodes : Vec<TrieNode>,
    ALPHA_SIZE : usize,
    DEFAULT : isize,
    REF_CHAR : char,
}

impl Trie {
    /// Generate empty trie
    pub fn new() -> Self { 
        let def = -1;
        let alpha_sz = 27;
        Self {
            nodes : vec![ TrieNode::build(false, def, alpha_sz) ],
            ALPHA_SIZE : alpha_sz,
            DEFAULT : def,
            REF_CHAR : 'a'
        }
     }

    /// Generate trie from dictionary
    pub fn build<'a>(dictionary : impl Iterator<Item=&'a str>) -> Self { 
        let mut trie = Trie::new();

        dictionary.for_each( |word : &str| 
            {
                trie.add(word.bytes());
            }
        
        );

        trie

     }

    /// Add a single word to trie
    fn add(self : &mut Self, word : impl Iterator<Item=u8>) { 
        let mut trie_idx : usize = 0; // idx of root node

        word.for_each( |sym : u8 | 
            {
                let node_idx = sym.wrapping_sub(self.REF_CHAR as u8) as usize;

                if self.nodes[trie_idx].next_char[node_idx] == DEFAULT  
                {
                    self.nodes.push( TrieNode::new(self.DEFAULT, self.ALPHA_SIZE) );
                    self.nodes[trie_idx].next_char[node_idx] = self.nodes.len().checked_sub(1).unwrap() as isize; 
                }
                trie_idx = self.nodes.len().checked_sub(1).unwrap();
            }
        );

        if let Some(last_node) = self.nodes.last_mut() 
        {
            last_node.is_terminal = true;
        }
    }

    /// Query if a word is on trie
    // TODO  Use iterators
    pub fn query(self : &Self, mut word : impl Iterator<Item=u8>) -> bool {
        let mut trie_idx : usize = 0;

        for sym in word
        {
            let node_idx  = sym.checked_sub(self.REF_CHAR as u8).unwrap() as usize;
            let next_idx = self.nodes[trie_idx].next_char[node_idx] ;

            if next_idx == DEFAULT {
                return false;
            }

            trie_idx = next_idx as usize;
        }

        self.nodes[trie_idx].is_terminal
    } 

    /// TODO Remove a word from trie
    pub fn remove(word : impl Iterator<Item=char>) { 

    }

    /// Generates lexicographically sorted vector of words in trie
    fn dbg (self: &Self, root_idx : usize, partial_word : & mut String, output : & mut Vec<String> ) { 
        let current_node : &TrieNode = &self.nodes[root_idx];
        
        // Gather words when required
        if current_node.is_terminal 
        {
            output.push( partial_word.clone() );
        }

        for (char_ref, &node_idx) in current_node.next_char.iter().enumerate()
        {
            if node_idx != DEFAULT 
            {
                let current_sym = (char_ref as u8).checked_add(self.REF_CHAR as u8).unwrap() as char ;

                partial_word.push( current_sym );
                self.dbg(node_idx as usize, partial_word, output); 
                partial_word.pop();
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    mod TrieNodeTests {
        use super::*;
        #[test]
        fn trie_node_constructor() {
            let def = -1;
            let alpha_sz = 27;
            let new_trie = TrieNode::new(def, alpha_sz);
            assert_eq!(new_trie.next_char, vec![def; alpha_sz]);
        }

        #[test]
        fn trie_node_adaptor() { 
            let def = -1;
            let alpha_sz = 27;
            let new_trie = TrieNode::build(true, def, alpha_sz);
            assert_eq!(new_trie.is_terminal, true);
         }
    }

    mod TrieTests {
        use super::*;


        #[test]
        fn trie_constructor() { 
            let def = -1;
            let alpha_sz = 27;
            let trie = Trie::new();
            assert_eq!(trie.nodes, vec![TrieNode::new(def, alpha_sz)]);
        }

        #[test]
        fn trie_adaptor() { 
            // Generate word list
            let words = "donde esta la biblioteca"; 

            let mut word_list = 
                words
                    .split_whitespace()
                    .map( |el| el.to_string()  ).collect::<Vec<String>>();
            word_list.sort();

            // Build trie
            let trie = Trie::build(words.split_whitespace());

            let mut partial_word : String = String::new();
            let mut result : Vec<String> = vec![];
            trie.dbg(0, &mut partial_word, &mut result);

            assert_eq!(result, word_list); 
            
         }

        #[test]
        fn trie_add() { 
            let def = -1;
            let alpha_sz = 27;
            let mut trie = Trie::new();
            let word = String::from("hola");


            trie.add(word.bytes());

            let mut root = TrieNode::new(def, alpha_sz);
            root.next_char[('h' as usize).wrapping_sub('a' as usize)] = 1;
            let mut h = TrieNode::new(def, alpha_sz);
            h.next_char[('o' as usize).wrapping_sub('a' as usize)] = 2;
            let mut o = TrieNode::new(def, alpha_sz);
            o.next_char[('l' as usize).wrapping_sub('a' as usize)] = 3;
            let mut l = TrieNode::new(def, alpha_sz);
            l.next_char[('a' as usize).wrapping_sub('a' as usize)] = 4;
            let mut a= TrieNode::build(true, def, alpha_sz);


            assert_eq!(trie.nodes, vec![root, h, o, l, a]);
            assert_eq!(trie.nodes.last().unwrap().is_terminal, true);


         }

        #[test]
        fn trie_query() { 
            let mut only_word = "prueba";
            let trie = Trie::build(only_word.split_whitespace());

            assert_eq!(trie.query(only_word.bytes()), true);
         }

        #[ignore]
        #[test]
        fn trie_remove() {todo!(); }

        }
    }