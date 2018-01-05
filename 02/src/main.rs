use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.lines().map(|l| l.split_whitespace().map(u32::from_str).collect::<Result<Vec<u32>, _>>()).collect::<Result<Vec<Vec<u32>>, _>>().unwrap();

    let mut total: u32 = 0;

    for row in &input {
        let (min, max) = min_and_max(row.iter());
        total += max - min;
    }
    println!("Part 1: {}", total);
    total = 0;

    'row: for row in &input {
        for v in row {
            for u in row {
                if v == u {continue}
                else if v % u == 0 {total += v / u; continue 'row}
                else if u % v == 0 {total += u / v; continue 'row}
            }
        }
    }
    println!("Part 2: {}", total);

}

fn min_and_max<'a, T>(input: T) -> (u32, u32) where T: IntoIterator<Item=&'a u32> {
    let mut max = 0;
    let mut min = u32::max_value();

    for val in input {
        if *val > max {max = *val}
        if *val < min {min = *val}
    }

    (min, max)
}
