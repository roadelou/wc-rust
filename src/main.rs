// Used to read from stdin line by line.
use std::io::{self, BufRead};

fn main() {
    // We first acquire the stdin.
    let (characters, words, lines) = io::stdin().lock().lines().filter_map(
        // For each line we get a Result that we have to handle. We simply skip
        // erroneous lines using filter_map.
        |maybe_line| maybe_line.ok()
    ).fold(
        // We count the characters, words and lines that we encounter. Each of
        // them starts at 0.
        (0, 0, 0),
        |(characters, words, lines), line| 
            // We increment the three important values.
            (
                // The number of characters can be derived from the length of
                // the string +1 because of the newline.
                characters + line.chars().count() + 1,
                // The number of words can be derived from splitting the string
                // along the ' ' character.
                words + line.split(' ').filter(
                    // Words must have non-zero length.
                    |possible_word| possible_word.len() > 0
                ).count(),
                // The number of lines is simply incremented.
                lines + 1,
            )

    );
    // We print our results.
    println!("Characters: {}", characters);
    println!("Words: {}", words);
    println!("Lines: {}", lines);
}
