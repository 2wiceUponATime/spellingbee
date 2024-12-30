use std::{fs::File, io::{self, BufRead, BufReader, Lines}};

#[derive(Debug)]
pub struct Letters {
    pub center: char,
    pub other: Vec<char>,
}

impl Letters {
    pub fn new(center: char, other: Vec<char>) -> Self {
        Letters {
            center,
            other,
        }
    }

    pub fn count(&self) -> usize {
        self.other.len() + 1
    }
}

pub fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader
        .lines();
    Ok(lines)
}

pub fn is_valid(word: &str) -> bool {
    if word.len() < 4 {
        return false;
    }
    let mut letters: Vec<char> = vec![];
    for letter in word.chars() {
        if !letters.contains(&letter) {
            letters.push(letter);
            if letters.len() > 7 {
                return false;
            }
        }
    }
    true
}

pub fn is_solution(word: &str, letters: &Letters) -> bool {
    // println!("{:?}", letters);
    if word.len() < 4 {
        return false;
    }
    if !word.contains(letters.center) {
        return false;
    }
    for letter in word.chars() {
        if letter == letters.center || letters.other.contains(&letter) {
            continue;
        }
        return false;
    }
    true
}

pub fn is_pangram(word: &str, letter_count: usize) -> bool {
    let mut letters: Vec<char> = vec![];
    for letter in word.chars() {
        if !letters.contains(&letter) {
            letters.push(letter);
            if letters.len() >= letter_count {
                return true
            }
        }
    }
    false
}

pub fn points(word: &str, letter_count: usize) -> usize {
    if word.len() <= 4 {
        return 1
    }
    if is_pangram(word, letter_count) {
        return word.len() + 7
    }
    word.len()
}