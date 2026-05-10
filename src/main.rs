use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct WordTree<'a> {
    children: HashMap<char, WordTree<'a>>,
    word: Option<&'a String>,
}
impl<'a> WordTree<'a> {
    fn add_word(&mut self, word: &'a String, current_idx: usize) {
        if let Some(next_char) = word.chars().nth(current_idx) {
            if let Some(child) = self.children.get_mut(&next_char) {
                child.add_word(&word, current_idx + 1);
            }
            else {
                let mut new_child = build_wordtree_node();
                new_child.add_word(word, current_idx + 1);
                self.children.insert(next_char, new_child);
            }
        }
        else {
            self.word = Some(word);
        }
    }

    fn find_words(&self, available_letters: &[char], found_words: &mut Vec<&'a String>) {
        if let Some(word) = self.word {
            found_words.push(word);
        }
        for letter in available_letters {
            if let Some(child) = self.children.get(letter) {
                child.find_words(available_letters, found_words);
            }
        }
    }
}
fn build_wordtree_node<'a>() -> WordTree<'a> {
    WordTree {
        children: HashMap::new(),
        word: None,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_word_list() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    match read_lines("./words.txt") {
        Ok(lines) => {
            for line in lines.map_while(Result::ok) {
                words.push(line);
            }
        }
        Err(e) => {
            eprintln!("Error opening words.txt: {}", e);
        }
    }
    words
}

fn main() {
    let words = read_word_list();
    let letters = ['l', 'o', 'b'];

    let mut word_tree = build_wordtree_node();
    for word in &words {
        word_tree.add_word(word, 0);
    }

    let mut found_words: Vec<&String> = Vec::new();
    word_tree.find_words(&letters, &mut found_words);
    println!("Vec: {:?}", found_words);
}
