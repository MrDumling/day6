use std::fs;

fn continuous_unique_index(searched_string: String, unique_length: usize) -> usize {
    for i in 0..searched_string.len() - unique_length {
        let sub_str = &searched_string[i..i+unique_length];

        if {
            let mut unique = std::collections::HashSet::new();
            sub_str.chars().all(|x| unique.insert(x))
        } {
            return i + unique_length;
        }
    }
    
    panic!("Unable to find substring with {unique_length} continuous unique characters.");
}

fn puzzle_1() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    println!("{}", continuous_unique_index(contents, 4));
}

fn puzzle_2() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    println!("{}", continuous_unique_index(contents, 14));
}

fn main() {
    puzzle_1();
    puzzle_2();
}
