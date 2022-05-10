use std::env;
use std::process;

const VOWELS: &str = "aeiouAEIOU";
const CONSONATS: & str = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";

const SYMBOLS_AND_NUMBERS: &[char] = &['1','2','3','4','5','6','7','8','9','0',
                                       '`','~','!','@','#','$','%',' ','&','*',
                                       '(',')','{','}','[',']','<','>','?',',',
                                       '.','\"',':',';','\'','|','\\','-','_',
                                       '+','='];

fn main() {
    let args: Vec<String> =  env::args().collect();

    if !is_english(&args[1]) {
        println!("{} is not english", args[1]);
        process::exit(-1);
    }

    let input = &args[1];

    println!("{}", to_pig_latin(input));
}

fn to_pig_latin(s: &str) -> String {
    let words = s.split(SYMBOLS_AND_NUMBERS).collect::<Vec<&str>>();
    let mut new_string = String::new();
    
    let mut previous_word_end = 0;

    for word in words { 
        if word.is_empty() { continue }

        let first_char: &str = &word[0..1];

        let mut word_end_index = word.len();

        let word_start_index = s[previous_word_end..].find(word).unwrap() + previous_word_end;

        if previous_word_end >= 1 {
            new_string.push_str(&s[previous_word_end..word_start_index]);  
            word_end_index = word_start_index + word.len();
        }

        previous_word_end = word_end_index;

        if is_vowel(first_char) {
            new_string.push_str(&format!("{}yay", word));
        }else if is_consonat(first_char) {
            let consonat_cluster = &word[..find_consonat_cluster(word)];
            let end_of_word_slice: &str = match word.len() {
                1 => "",
                _ => &word[consonat_cluster.len()..],
            };
            new_string.push_str(&format!("{}{}ay", end_of_word_slice, consonat_cluster));
        }
    }

    new_string
}

// checks if a string has a vowel in it
fn is_vowel(s: &str) -> bool {
    VOWELS.chars().any(|c| c.to_string() == s[0..1])
}

// will return the index of a of the end of consonats ie "should" would return 2 returns zero if not consonat cluster exits
fn find_consonat_cluster(s: &str) -> usize {
    let chars = s.chars();

    for (index, c) in chars.enumerate() {
        if is_vowel(&c.to_string()) { return index }
    }

    0
}

fn is_consonat(s: &str)-> bool {
    CONSONATS.chars().any(|c| c.to_string() == s[0..1])
}

fn is_english(string: &str) -> bool {
    string.chars().all(|c| matches!(c, ' '..='~'))
}

