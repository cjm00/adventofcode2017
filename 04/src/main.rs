use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


fn part1(input: &str) -> u32 {
    let mut count: u32 = 0;

    for line in input.lines() {
        if has_no_duplicates(line) {
            count += 1;
        }
    }
    count
}

fn has_no_duplicates(input: &str) -> bool {
    let mut set = HashSet::new();

    for blob in input.split_whitespace() {
        if !set.insert(blob) {
            return false;
        }
    }
    true
}

fn part2(input: &str) -> u32 {
    let mut count: u32 = 0;

    for line in input.lines() {
        if has_no_anagrams(line) {
            count += 1;
        }
    }
    count
}

fn has_no_anagrams(input: &str) -> bool {
    let mut set = HashSet::new();
    for blob in input.split_whitespace() {
        let soup = LetterSoup::from_str(blob).unwrap();
        if !set.insert(soup) {
            return false;
        }
    }
    true
}

#[derive(PartialEq, Eq, Hash)]
struct LetterSoup([u8; 26]);

impl FromStr for LetterSoup {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.is_ascii() {
            return Err(());
        }

        let mut char_counter: [u8; 26] = [0; 26];
        for l in input.trim().as_bytes() {
            char_counter[(l - 97) as usize] += 1;
        }
        Ok(LetterSoup(char_counter))
    }
}