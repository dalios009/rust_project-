use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    println!("Text Analyzer - Enter your text:");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let words = tokenize(&input_text);
    
    let word_count = words.len();
    
    let word_frequency = calculate_word_frequency(&words);
    
    println!("Statistics:");
    println!("Word Count: {}", word_count);
    println!("Word Frequency:");
    
    for (word, count) in &word_frequency {
        println!("{}: {}", word, count);
    }

    let most_frequent_words = find_most_frequent_words(&word_frequency, 5);
    println!("Most Frequent Words:");
    for (word, count) in &most_frequent_words {
        println!("{}: {}", word, count);
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

fn calculate_word_frequency(words: &[String]) -> HashMap<String, u32> {
    let mut word_frequency: HashMap<String, u32> = HashMap::new();
    
    for word in words {
        let entry = word_frequency.entry(word.to_string()).or_insert(0);
        *entry += 1;
    }
    
    word_frequency
}

fn find_most_frequent_words(word_frequency: &HashMap<String, u32>, count: usize) -> Vec<(String, u32)> {
    let mut word_count_pairs: Vec<(String, u32)> = word_frequency.iter().map(|(word, count)| (word.clone(), *count)).collect();
    word_count_pairs.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count in descending order
    word_count_pairs.truncate(count);
    word_count_pairs
}