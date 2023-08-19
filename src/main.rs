use std::collections::{HashMap, HashSet}; // Import necessary modules from the standard library
use std::io; // Import module for input/output operations

fn main() {
    println!("Text Analyzer - Enter your text:"); // Prompt user for input

    let mut input_text = String::new(); // Initialize a mutable string to hold user input
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line"); // Read user input from stdin and handle any errors

    let words = tokenize(&input_text); // Tokenize the input text into individual words
    
    let word_count = words.len(); // Calculate the total word count
    
    let word_frequency = calculate_word_frequency(&words); // Calculate the frequency of each word
    
    println!("Statistics:");
    println!("Word Count: {}", word_count); // Print the total word count
    println!("Word Frequency:");
    
    for (word, count) in &word_frequency {
        println!("{}: {}", word, count); // Print word frequency for each word
    }

    let most_frequent_words = find_most_frequent_words(&word_frequency, 5); // Find the most frequent words
    println!("Most Frequent Words:");
    for (word, count) in &most_frequent_words {
        println!("{}: {}", word, count); // Print the most frequent words and their counts
    }
}

// Tokenizes the input text into a vector of words
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace() // Splits the input text into words using whitespace as delimiter
        .map(|word| word.to_string()) // Converts each word to a String
        .collect() // Collects the words into a vector
}

// Calculates the frequency of each word in the provided vector
fn calculate_word_frequency(words: &[String]) -> HashMap<String, u32> {
    let mut word_frequency: HashMap<String, u32> = HashMap::new(); // Creates a HashMap to store word frequencies
    
    for word in words {
        let entry = word_frequency.entry(word.to_string()).or_insert(0); // Retrieves or inserts a word into the HashMap
        *entry += 1; // Increments the word's frequency
    }
    
    word_frequency // Returns the word frequency HashMap
}

// Finds the most frequent words based on the provided word frequency HashMap
fn find_most_frequent_words(word_frequency: &HashMap<String, u32>, count: usize) -> Vec<(String, u32)> {
    let mut word_count_pairs: Vec<(String, u32)> = word_frequency.iter().map(|(word, count)| (word.clone(), *count)).collect(); // Creates a vector of word-frequency pairs
    word_count_pairs.sort_by(|a, b| b.1.cmp(&a.1)); // Sorts the pairs by frequency in descending order
    word_count_pairs.truncate(count); // Keeps only the top N pairs
    word_count_pairs // Returns the vector of most frequent word-frequency pairs
}
