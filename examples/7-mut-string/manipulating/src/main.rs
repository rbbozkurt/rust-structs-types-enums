/*
    this function should count the number of each vowel and prints the result
*/
fn count_vowels(sentence: &str) {
    let mut vowel_counts = (0, 0, 0, 0, 0);

    for c in sentence.chars() {
        match c {
            'a' => vowel_counts.0 += 1,
            'e' => vowel_counts.1 += 1,
            'i' => vowel_counts.2 += 1,
            'o' => vowel_counts.3 += 1,
            'u' => vowel_counts.4 += 1,
            _ => continue,
        }
    }

    println!("a: {}, e: {}, i: {}, o: {}, u: {}", vowel_counts.0, vowel_counts.1, vowel_counts.2, vowel_counts.3, vowel_counts.4);
}


fn longest_word(sentence: &str) -> &str {
    let words = sentence.split(' ');
    let mut longest = "";
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}
fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
    count_vowels(&sentence);
    let longest_word = longest_word(&sentence);
    println!("{}", longest_word);
}
