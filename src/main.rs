use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::env;

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

    fn find_words(&self, available_letters: &[char], mandatory_letter: char, found_words: &mut Vec<&'a String>) {
        if let Some(word) = self.word {
            if let Some(_) = word.find(mandatory_letter) {
                found_words.push(word);
            }
        }
        for letter in available_letters {
            if let Some(child) = self.children.get(letter) {
                child.find_words(available_letters, mandatory_letter, found_words);
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

fn parse_args(args: &[String]) -> (Vec<char>, char) {
    if args.len() < 2 {
        panic!("Must supply blossom letters")
    }
    let mut letters = Vec::new();
    let mut first = true;
    for arg in args {
        if first {
            first = false;
            continue;
        }
        if arg.len() != 1 {
            panic!("All args must be single characters");
        }
        letters.push(arg.chars().next().unwrap());
    }
    let mandatory_letter = letters[0];
    return (letters, mandatory_letter);
}

fn main() {
    let words = read_word_list();
    let args: Vec<String> = env::args().collect();
    let (letters, mandatory_letter) = parse_args(&args);

    let mut word_tree = build_wordtree_node();
    for word in &words {
        word_tree.add_word(word, 0);
    }

    let mut found_words: Vec<&String> = Vec::new();
    word_tree.find_words(&letters, mandatory_letter, &mut found_words);
    println!("Vec: {:?}", found_words);
}
