use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut board: Vec<i32> = input.split_whitespace().map(i32::from_str).collect::<Result<Vec<i32>, _>>().unwrap();
    let mut count: u32 = 0;
    let mut index: i32 = 0;
    let mut instruction: i32;

    while let Some(i) = board.get_mut(index as usize) {
        count += 1;
        instruction = *i;
        *i += 1;
        index += instruction;
    }
    count
}

fn part2(input: &str) -> u32 {
    let mut board: Vec<i32> = input.split_whitespace().map(i32::from_str).collect::<Result<Vec<i32>, _>>().unwrap();
    let mut count: u32 = 0;
    let mut index: i32 = 0;
    let mut instruction: i32;

    while let Some(i) = board.get_mut(index as usize) {
        count += 1;
        instruction = *i;
        if instruction >= 3 {
            *i -= 1;
        } else {
            *i += 1;
        }
        index += instruction;
    }
    count
}
