use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::time::Instant;

fn letter_index(c: &char) -> Result<usize, String> {
    if c.is_ascii_lowercase() {
        Ok((*c as usize) - ('a' as usize))
    } else {
        Err(format!("expected lowercase letter, got '{}'", c))
    }
}

fn word_to_idx_vec(w: &String) -> Result<Vec<usize>, String> {
    let mut result: Vec<usize> = Vec::new();
    for c in w.chars() {
        result.push(letter_index(&c)?);
    }
    result.sort();

    let mut deduped_result = Vec::new();
    let mut last_char: usize = 26 + 1;
    for ch in result {
        if ch != last_char {
            last_char = ch;
            deduped_result.push(ch);
        }
    }
    return Ok(deduped_result);
}

struct WordTree<'a> {
    children: [Option<Box<WordTree<'a>>>; 26],
    words: Option<Vec<&'a String>>,
}
impl<'a> WordTree<'a> {
    fn add_word(&mut self, word_str: &'a String, mut word: Option<Vec<usize>>, current_idx: usize) {
        if word.is_none() {
            match word_to_idx_vec(word_str) {
                Ok(idx_vec) => {
                    word = Some(idx_vec);
                }
                Err(_) => {
                    println!("Skipping {} as it has bad characters", word_str);
                    return;
                }
            }
        }
        let unwrapped_word = word.unwrap();
        if current_idx < unwrapped_word.len() {
            let c = unwrapped_word[current_idx];
            if self.children[c].is_none() {
                self.children[c] = Some(Box::new(build_wordtree_node()));
            }
            self.children[c].as_mut().unwrap().add_word(word_str, Some(unwrapped_word), current_idx + 1);
        }
        else {
            if self.words.is_none() {
                self.words = Some(Vec::new());
            }
            self.words.as_mut().unwrap().push(word_str);
        }
    }

    fn find_words(&self, available_letters: &[usize], mandatory_letter: char, found_words: &mut Vec<&'a String>) {
        if let Some(words) = &self.words {
            let test_word = &words[0];
            if let Some(_) = test_word.find(mandatory_letter) {
                found_words.extend(words.iter().copied());
            }
        }
        for letter in available_letters {
            if let Some(child) = &self.children[*letter] {
                child.find_words(available_letters, mandatory_letter, found_words);
            }
        }
    }
}
fn build_wordtree_node<'a>() -> WordTree<'a> {
    WordTree {
        children: [const { None }; 26],
        words: None,
    }
}

fn simple_baseline<'a>(word_list: &'a Vec<String>, letters: &HashSet<char>, mandatory_letter: char, found_words: &mut Vec<&'a String>) {
    for word in word_list {
        let mut valid = true;
        let mut mandatory_valid = false;
        for c in word.chars() {
            if !letters.contains(&c) {
                valid = false;
                break;
            }
            if c == mandatory_letter {
                mandatory_valid = true;
            }
        }
        if valid & mandatory_valid {
            found_words.push(word);
        }
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

fn parse_input(input: &str) -> (Vec<char>, char, HashSet<char>) {
    if input.len() < 1 {
        panic!("Must supply blossom letters")
    }
    let mut letters: Vec<char> = Vec::new();
    let mandatory_letter = input.chars().next().unwrap();
    let mut letter_set: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        letters.push(c);
        letter_set.insert(c);
    }
    return (letters, mandatory_letter, letter_set);
}

fn sort_output(word_list: &mut Vec<&String>) {
    word_list.sort_by_key(|w| w.len());
    word_list.reverse();
}

#[allow(unreachable_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let words = read_word_list();
    let mut word_tree = build_wordtree_node();
    for word in &words {
        word_tree.add_word(word, None, 0);
    }
    println!("Loaded the word list in {} milliseconds", start.elapsed().as_millis());

    loop {
        println!("Input all letters, center letter first, lowercase and without spaces");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let (letters, mandatory_letter, letter_set) = parse_input(&user_input);

        let search_start = Instant::now();
        let mut found_words: Vec<&String> = Vec::new();
        let letter_idxs: Vec<usize> = letters.iter().map(letter_index).collect::<Result<Vec<_>, _>>()?;
        word_tree.find_words(&letter_idxs, mandatory_letter, &mut found_words);
        sort_output(&mut found_words);
        println!("Found words: {:?}", found_words);
        println!("Completed in {} microseconds", search_start.elapsed().as_micros());

        let baseline_start = Instant::now();
        let mut baseline_words: Vec<&String> = Vec::new();
        simple_baseline(&words, &letter_set, mandatory_letter, &mut baseline_words);
        sort_output(&mut baseline_words);
        println!("Baseline words: {:?}", baseline_words);
        println!("Baseline completed in {} milliseconds", baseline_start.elapsed().as_millis());
    }

    Ok(())
}
