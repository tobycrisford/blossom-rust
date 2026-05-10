use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;

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

fn parse_input(input: &str) -> (Vec<char>, char) {
    if input.len() < 1 {
        panic!("Must supply blossom letters")
    }
    let mut letters: Vec<char> = Vec::new();
    let mandatory_letter = input.chars().next().unwrap();
    for c in input.chars() {
        letters.push(c);
    }
    return (letters, mandatory_letter);
}

fn main() {
    let start = Instant::now();
    let words = read_word_list();
    let mut word_tree = build_wordtree_node();
    for word in &words {
        word_tree.add_word(word, 0);
    }
    println!("Loaded the word list in {} milliseconds", start.elapsed().as_millis());

    loop {
        println!("Input all letters, center letter first, lowercase and without spaces");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let (letters, mandatory_letter) = parse_input(&user_input);

        let search_start = Instant::now();
        let mut found_words: Vec<&String> = Vec::new();
        word_tree.find_words(&letters, mandatory_letter, &mut found_words);
        found_words.sort_by_key(|w| w.len());
        found_words.reverse();
        println!("Found words: {:?}", found_words);
        println!("Completed in {} milliseconds", search_start.elapsed().as_millis());
    }
}
