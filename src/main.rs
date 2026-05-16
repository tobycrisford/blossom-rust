use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::time::Instant;
use std::collections::HashMap;

const NUM_CHARS: usize = 26;
const INPUT_SIZE: usize = 3;

fn letter_index(c: &char) -> Result<usize, String> {
    if c.is_ascii_lowercase() {
        Ok((*c as usize) - ('a' as usize))
    } else {
        Err(format!("expected lowercase letter, got '{}'", c))
    }
}

fn index_to_letter(i: usize) -> Result<char, String> {
    if i < NUM_CHARS {
        Ok(char::from(b'a' + i as u8))
    } else {
        Err(format!("expected index in range 0..NUM_CHARS, got {}", i))
    }
}

fn word_to_idx_vec(w: &String) -> Result<Vec<usize>, String> {
    let mut result: Vec<usize> = Vec::new();
    for c in w.chars() {
        result.push(letter_index(&c)?);
    }
    result.sort();

    let mut deduped_result = Vec::new();
    let mut last_char: usize = NUM_CHARS + 1;
    for ch in result {
        if ch != last_char {
            last_char = ch;
            deduped_result.push(ch);
        }
    }
    return Ok(deduped_result);
}

struct WordTree<'a> {
    children: [Option<Box<WordTree<'a>>>; NUM_CHARS],
    words: Vec<&'a String>,
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
            self.words.push(word_str);
        }
    }

    fn find_words(&self, available_letters: &[usize], mandatory_letter: char, found_words: &mut Vec<&'a String>) {
        if self.words.len() > 0 {
            let test_word = &self.words[0];
            if let Some(_) = test_word.find(mandatory_letter) {
                found_words.extend(self.words.iter().copied());
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
        children: [const { None }; NUM_CHARS],
        words: Vec::new(),
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



fn sort_output(word_list: &mut Vec<&String>) {
    word_list.sort_by_key(|w| w.len());
    word_list.reverse();
}

fn solve_all_blossoms<'a>(
    results: &mut HashMap<Vec<usize>, Vec<&'a String>>,
    word_tree: &WordTree<'a>,
    current_letters: &mut Vec<usize>,
    current_idx: usize
) -> Result<(), String> {
    let mut start_char = 0;
    if current_idx > 0 {
        start_char = current_letters[current_idx - 1] + 1;
    }
    if !(start_char < NUM_CHARS) {
        return Ok(());
    }
    for ch in start_char..NUM_CHARS {
        current_letters[current_idx] = ch;
        if current_idx < current_letters.len() - 1 {
            let _ = solve_all_blossoms(results, word_tree, current_letters, current_idx + 1);
        }
        else if current_idx == current_letters.len() - 1 {
            for mandatory_letter in &*current_letters {
                let mandatory_letter_char = index_to_letter(*mandatory_letter)?;
                let mut found_words: Vec<&String> = Vec::new();
                word_tree.find_words(current_letters, mandatory_letter_char, &mut found_words);
                sort_output(&mut found_words);
                let mut lookup_key = current_letters.clone();
                lookup_key.push(*mandatory_letter);
                results.insert(lookup_key, found_words);
            }
        }
    }

    Ok(())
}

fn build_word_tree_from_words(words: &Vec<String>) -> WordTree<'_> {
    let mut word_tree = build_wordtree_node();
    for word in words {
        word_tree.add_word(word, None, 0);
    }
    return word_tree;
}

pub trait BlossomSolver {
    fn solve(&self, available_letters: &[char], mandatory_letter: char) -> Result<Vec<& String>, String>;

    fn solve_with_timing(&self, available_letters: &[char], mandatory_letter: char) -> Result<(Vec<& String>, u128), String> {
        let start = Instant::now();
        let result = self.solve(available_letters, mandatory_letter)?;
        let elapsed_time = start.elapsed().as_micros();
        return Ok((result, elapsed_time));
    }
}

struct TreeSolver<'a> {
    word_tree: WordTree<'a>
}
impl BlossomSolver for TreeSolver<'_> {
    fn solve(&self, available_letters: &[char], mandatory_letter: char) -> Result<Vec<& String>, String> {
        let letter_idxs: Vec<usize> = available_letters.iter().map(letter_index).collect::<Result<Vec<_>, _>>()?;
        let mut found_words: Vec<& String> = Vec::new();
        self.word_tree.find_words(&letter_idxs, mandatory_letter, &mut found_words);
        sort_output(&mut found_words);
        return Ok(found_words);
    }
}

struct BaselineSolver<'a> {
    words: &'a Vec<String>
}
impl BlossomSolver for BaselineSolver<'_> {
    fn solve(&self, available_letters: &[char], mandatory_letter: char) -> Result<Vec<& String>, String> {
        let mut letter_set: HashSet<char> = HashSet::new();
        for letter in available_letters {
            letter_set.insert(*letter);
        }
        let mut found_words: Vec<& String> = Vec::new();
        for word in self.words {
            let mut valid = true;
            let mut mandatory_valid = false;
            for c in word.chars() {
                if !letter_set.contains(&c) {
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
        sort_output(&mut found_words);
        return Ok(found_words);
    }
}

struct LookupSolver<'a> {
    all_blossoms: HashMap<Vec<usize>, Vec<&'a String>>
}
impl BlossomSolver for LookupSolver<'_> {
    fn solve(&self, available_letters: &[char], mandatory_letter: char) -> Result<Vec<& String>, String> {
        let mut letter_idxs: Vec<usize> = available_letters.iter().map(letter_index).collect::<Result<Vec<_>, _>>()?;
        letter_idxs.sort();
        letter_idxs.push(letter_index(&mandatory_letter)?);
        match self.all_blossoms.get(&letter_idxs) {
            None => Err("No entry found in lookup".to_string()),
            Some(result) => Ok(result.clone()),
        }
    }
}


fn parse_input(input: &str) -> (Vec<char>, char) {
    if input.len() < 1 {
        panic!("Must supply blossom letters")
    }
    let mut letters: Vec<char> = Vec::new();
    let mandatory_letter = input.chars().next().unwrap();
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        letters.push(c);
    }
    return (letters, mandatory_letter);
}


#[allow(unreachable_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Select solution mode out of: tree, lookup, baseline");
    println!("If you select 'lookup', input size is hardcoded as {}", INPUT_SIZE);
    let mut solve_mode = String::new();
    io::stdin()
        .read_line(&mut solve_mode)
        .expect("Failed to read line");
    let solve_mode = solve_mode.trim();

    let construction_start = Instant::now();
    let words = read_word_list();
    let solver: Box<dyn BlossomSolver>;
    if solve_mode == "tree" {
        solver = Box::new(
            TreeSolver {
                word_tree: build_word_tree_from_words(&words),
            }
        );
    }
    else if solve_mode == "lookup" {
        let word_tree = build_word_tree_from_words(&words);
        let mut all_blossoms: HashMap<Vec<usize>, Vec<&String>> = HashMap::new();
        let _ = solve_all_blossoms(&mut all_blossoms, &word_tree, &mut vec![0; INPUT_SIZE], 0);
        solver = Box::new(
            LookupSolver {
                all_blossoms: all_blossoms,
            }
        );
    }
    else if solve_mode == "baseline" {
        solver = Box::new(
            BaselineSolver {
                words: &words,
            }
        );
    }
    else {
        println!("Solution mode you selected: {}", solve_mode);
        panic!("Bad solution mode selected")
    }
    println!("Loaded the solver in {} milliseconds", construction_start.elapsed().as_millis());

    loop {
        println!("Input all letters, center letter first, lowercase and without spaces");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let (letters, mandatory_letter) = parse_input(&user_input);

        let (solution, elapsed_time) = solver.solve_with_timing(&letters, mandatory_letter)?;

        println!("Found words: {:?}", solution);
        println!("Completed in {} microseconds", elapsed_time);
    }

    Ok(())
}
