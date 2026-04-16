use std::collections::HashMap;
use std::io;

fn read_input() -> String {
    let mut input = String::new();
    // let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let parsed_input: String = input.trim().to_string();
    return parsed_input;
}

fn preprocess(parsed_string: &String) -> String {
    // for c in parsed_input.chars() {
    //     if c.is_ascii_punctuation() {
    //         input_string.push(' ');
    //     } else {
    //         input_string.push(c);
    //     }
    // }

    let cleaned: String = parsed_string
        .chars()
        .map(|c| if c.is_ascii_punctuation() { ' ' } else { c })
        .collect();

    return cleaned;
}

fn split_into_words(input: &String) -> Vec<&str> {
    let words: Vec<&str> = input.split_whitespace().collect();
    // println!("splitted words : {:?}", words);
    words
}

fn count_words(wordlist: &Vec<&str>) -> usize {
    let count = wordlist.iter().count();
    // println!("count is : {}", count);
    count
}

fn count_character(wordlist: &Vec<&str>) -> usize {
    let mut sum = 0;

    for s in wordlist {
        sum += s.len();
    }
    sum
}

fn read_multiple_lines() -> String {
    let mut multiple_lines = String::new();

    loop {
        let line = read_input();
        if line.is_empty() {
            break;
        } else {
            multiple_lines.push_str(&line);
        }
    }

    multiple_lines
}

fn count_frequency(listofwords: &String) {
    let mut frequency_hashmap = HashMap::new();

    for word in listofwords.split_whitespace() {
        let count = frequency_hashmap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("frequencies : {:?}", frequency_hashmap);
}

fn characters_frequency(listofwords1: &String) {
    let mut character_hashmap = HashMap::new();

    for c in listofwords1.chars() {
        if c == ' ' {
        } else {
            let count = character_hashmap.entry(c).or_insert(0);
            *count += 1;
        }
    }
    println!("frequencies : {:?}", character_hashmap);
}

fn longestword(wordlist1: &Vec<&str>) {
    if let Some(longest) = wordlist1.iter().max_by_key(|w| w.len()) {
        println!("longest word : {}", longest);
    }
}

fn main() {
    println!(
        "reading multiple lines so enter multiple lines, hit enter twice to submit your text."
    );
    let multiplelines = read_multiple_lines();
    println!("multiplelines are : {}", multiplelines);

    println!("enter text : ");
    let input_text = read_input();
    println!("input text : {}", input_text);

    let preprocessed_text = preprocess(&multiplelines);
    println!("preprocessed text : {}", preprocessed_text);

    let words_list = split_into_words(&preprocessed_text);
    println!("word list is : {:?}", words_list);

    let countword = count_words(&words_list);
    println!("no of words are : {}", countword);

    let countcharcters = count_character(&words_list);
    println!("no of characters are : {}", countcharcters);

    count_frequency(&preprocessed_text);
    characters_frequency(&preprocessed_text);
    longestword(&words_list);
}
