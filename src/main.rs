use std::{env::args, fs::File, io::{self, stdin, stdout, Write}};
use spellingbee::{is_pangram, is_solution, is_valid, points, read_lines, Letters};

struct Solutions {
    solutions: Vec<(usize, String)>,
    total_points: usize,
    letters: Letters,
}

fn trim() {
    let args: Vec<String> = args().collect();
    let lines: Vec<io::Result<String>> = read_lines(&args[2])
        .unwrap()
        .collect();

    let mut output = File::create(&args[3]).unwrap();
    let length = lines.len();
    let padding = length.to_string().len();
    let mut word_number = 0;
    for line in lines {
        let word = line.unwrap().to_lowercase();
        word_number += 1;
        print!("\rprocessing word {word_number:0>padding$} of {length}...");
        if !is_valid(&word) {
            continue;
        }
        writeln!(output, "{}", word).unwrap();
    }
    println!("");
}

fn get_solutions() -> Solutions {
    let mut solutions: Vec<(usize, String)> = vec![];
    let mut total_points = 0;

    let args: Vec<String> = args().collect();
    let center = args[3].chars().next().unwrap();
    let letters = Letters::new(
        center,
        args[3].replace(center, "").chars().collect()
    );
    let lines: Vec<io::Result<String>> = read_lines(&args[2])
        .unwrap()
        .collect();

    
    let length = lines.len();
    let padding = length.to_string().len();
    let mut word_number = 0;
    for line in lines {
        let word = line.unwrap();
        let points = points(&word, letters.count());
        word_number += 1;
        print!("\rprocessing word {word_number:0>padding$} of {length}...");
        if is_solution(&word, &letters) {
            total_points += points;
            solutions.push((points, word));
        }
    }
    println!("");
    solutions.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        if a.0 > b.0 { return Less }
        if a.0 < b.0 { return Greater }
        if a.1 > b.1 { return Greater }
        if a.1 < b.1 { return Less }
        Equal
    });
    return Solutions {
        solutions,
        total_points,
        letters,
    };
}

fn solve() {
    let solutions = get_solutions();
    let words = solutions.solutions;
    let total_points = solutions.total_points;
    let letters = solutions.letters;

    println!("\n{total_points} points ({} words) found:", words.len());
    for (points, mut word) in words {
        if is_pangram(&word, letters.count()) {
            word = format!("**{word}**");
        }
        let plural = match points == 1 {
            true => "",
            false => "s",
        };
        println!("  {word}: {points} point{plural}");
    }
}

fn play() {
    let args: Vec<String> = args().collect();
    let center = args[3].chars().next().unwrap();
    let letters = Letters::new(
        center,
        args[3].replace(center, "").chars().collect()
    );
    let lines = read_lines(&args[2])
        .unwrap();
    let valid_words: Vec<String> = lines.map(
        |line| line.unwrap().to_lowercase()
    ).collect();
    let mut found_words: Vec<String> = vec![];
    let mut total_points = 0;
    let goal_points = get_solutions().total_points;

    println!("{goal_points} points found");
    println!("use /quit or /exit to quit and /words to see found words");
    loop {
        let plural = match total_points == 1 {
            true => "",
            false => "s",
        };
        print!("{total_points} total point{plural}: ");
        stdout().flush().unwrap();
        let mut word = String::new();
        stdin().read_line(&mut word).unwrap();
        word = String::from(word.trim());
        match word.as_str() {
            "/exit" | "/quit" => break,
            "/words" => {
                found_words.sort();
                println!("{}", found_words.join(", "));
                continue;
            },
            _ => (),
        }
        if !valid_words.contains(&word) {
            println!("not in word list");
            continue;
        }
        if !is_solution(&word, &letters) {
            println!("invalid letters");
            continue;
        }
        if found_words.contains(&word) {
            println!("already found");
            continue;
        }
        let points = points(&word, letters.count());
        total_points += points;
        found_words.push(word.clone());
        let plural = match points == 1 {
            true => "",
            false => "s",
        };
        if is_pangram(&word, letters.count()) {
            word = format!("**{word}**");
        }
        println!("{word}: {points} point{plural}");
        println!("{}/{goal_points} total points", total_points)
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut show_info = false;
    if args.len() >= 4 {
        match args[1].as_str() {
            "trim" => trim(),
            "solve" => solve(),
            "play" => play(),
            _ => { show_info = true; },
        }
    } else {
        show_info = true;
    }
    if show_info {
        let default = String::from("");
        let command = args.get(1).unwrap_or(&default);
        println!("usage: spellingbee {command}{}", match command.as_str() {
            "trim"  => " <dict_path> <target_path>",
            "solve" => " <dict_path> <letters>",
            "play"  => " <dict_path> <letters>",
            _ => "trim|solve|play",
        });
    }
}