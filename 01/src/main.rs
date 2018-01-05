fn main() {
    let input = include_str!("../input.txt");
    let trimmed = input.trim();
    let trimmed_bytes = trimmed.as_bytes();

    let mut total: u32 = 0;

    for (l, r) in trimmed.chars().zip(trimmed.chars().skip(1).chain(trimmed.chars().take(1))) {
        if l == r {
            total += l.to_digit(10).unwrap();
        }
    }

    println!("Part 1: {}", total);

    let len = trimmed.len();
    let half_len = len / 2;
    total = 0;

    for (i, v) in trimmed_bytes.into_iter().enumerate() {
        if *v == trimmed_bytes[(i + half_len) % len] {
            total += *v as u32 - 48;
        }
    }

    println!("Part 2: {}", total);

}
