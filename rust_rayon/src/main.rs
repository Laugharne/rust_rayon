#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

fn main() {
	// Read the contents of the file "lorem.txt"
	match fs::read_to_string("lorem.txt") {
		Ok(contents) => {
			// Remove '.' and ',' characters from the contents
			let cleaned_contents: String = contents.replace(".", "").replace(",", "");

			// Split the cleaned contents into words
			let words: Vec<&str> = cleaned_contents.split_whitespace().collect();

			// Count occurrences of each word in parallel
			let word_count: HashMap<String, usize> = words
				.par_iter()
				.map(|&word| {
					let mut map: HashMap<String, usize> = HashMap::new();
					*map.entry(word.to_string()).or_insert(0) += 1;
					map
				})
				.reduce(HashMap::new, |mut map_final: HashMap<String, usize>, map_thread: HashMap<String, usize>| {
					for (word, count) in map_thread {
						*map_final.entry(word).or_insert(0) += count;
					}
					map_final
				});

			// Print the word counts
			word_count.iter().for_each(|(word, count)| {
				println!("{}: {}", word, count);
			});

			println!("\nNbr words: {}", word_count.len());

			// Get the occurrence of a specific word
			let specific_word = "efficitur";
			match word_count.get(specific_word) {
				Some(&count) => println!("The word '{}' occurs {} times.", specific_word, count),
				None => println!("The word '{}' does not occur in the text.", specific_word),
			}
		}
		Err(e) => {
			// Handle the error if the file cannot be read
			eprintln!("Error reading file: {}", e);
		}
	}
}