use std::cmp;

fn main() {
    println!("Part 1: {}", part1(312051));
    println!("Part 2: {}", part2(312051));
}


fn part1(input: u32) -> u32 {
    let mut spiral = UlamSpiral::new();
    let coordinates = spiral.find_coordinates(input);
    manhattan_distance(coordinates)
}

fn part2(input: u32) -> u32 {
    let mut spiral = StressedSpiral::new();
    while !spiral.last_val_big_enough(input) {
        spiral.grow();
    }
    spiral.last()
}

const ULAM_GROWTH_RATE: u32 = 8;

struct UlamSpiral {
    topright: u32,
    topleft: u32,
    botleft: u32,
    botright: u32,
    layer: u32,
}

impl UlamSpiral {
    fn new() -> Self {
        UlamSpiral{
            topright: 1,
            topleft: 1,
            botleft: 1,
            botright: 1,
            layer: 0,
        }
    }
    
    fn grow(&mut self) {
        self.topright += 2 + self.layer * ULAM_GROWTH_RATE;
        self.topleft  += 4 + self.layer * ULAM_GROWTH_RATE;
        self.botleft  += 6 + self.layer * ULAM_GROWTH_RATE;
        self.botright += 8 + self.layer * ULAM_GROWTH_RATE;

        self.layer    += 1;
    }
    
    fn shrink(&mut self) {
        if self.layer == 0 { return }
        self.layer -= 1;

        self.topright -= 2 + self.layer * ULAM_GROWTH_RATE;
        self.topleft  -= 4 + self.layer * ULAM_GROWTH_RATE;
        self.botleft  -= 6 + self.layer * ULAM_GROWTH_RATE;
        self.botright -= 8 + self.layer * ULAM_GROWTH_RATE;

    }

    fn within_spiral(&self, n: u32) -> bool {
        n <= self.botright
    }

    fn grow_until_on_boundary(&mut self, n: u32) {
        while !self.within_spiral(n) {
            self.grow();
        }
    }

    fn find_coordinates(&mut self, n: u32) -> (i32, i32) {
        while self.within_spiral(n) {
            self.shrink();
        }
        self.grow_until_on_boundary(n);

        let layer = self.layer as i32;

        if n <= self.topright {
            let mut coord = (layer, layer);
            let mut tmp = self.topright;
            while tmp != n {
                tmp -= 1;
                coord.1 -= 1;
            }
            return coord;

        } else if n <= self.topleft {
            let mut coord = (-layer, layer);
            let mut tmp = self.topleft;
            while tmp != n {
                tmp -= 1;
                coord.0 += 1;
            }
            return coord;

        } else if n <= self.botleft {
            let mut coord = (-layer, -layer);
            let mut tmp = self.botleft;
            while tmp != n {
                tmp -= 1;
                coord.1 += 1;
            }
            return coord;
        } else {
            let mut coord = (layer, -layer);
            let mut tmp = self.botright;
            while tmp != n {
                tmp -= 1;
                coord.0 -= 1;
            }
            return coord;
        }
    }

    fn find_index(&mut self, coords: (i32, i32)) -> u32 {
        if coords == (0, 0) { return 1 }
        let (x, y) = coords;
        debug_assert_ne!(x, i32::min_value());
        debug_assert_ne!(y, i32::min_value());
        let target_layer = cmp::max(x.abs(), y.abs()) as u32;
        self.adjust_to_layer(target_layer);
        let layer = self.layer as i32;

        if x == layer {
            if coords == (layer, -layer) {return self.botright}
            let mut loc = (layer, layer);
            let mut index = self.topright;
            while loc != coords {
                loc.1 -= 1;
                index -= 1;
            }
            return index;
        } else if y == layer {
            let mut loc = (-layer, layer);
            let mut index = self.topleft;
            while loc != coords {
                loc.0 += 1;
                index -= 1;
            }
            return index;
        } else if x == -layer {
            let mut loc = (-layer, -layer);
            let mut index = self.botleft;
            while loc != coords {
                loc.1 += 1;
                index -= 1;
            }
            return index;
        } else {
            let mut loc = (layer, -layer);
            let mut index = self.botright;
            while loc != coords {
                loc.0 -= 1;
                index -= 1;
            }
            return index;
        }
    }

    fn adjust_to_layer(&mut self, layer: u32) {
        while layer > self.layer {
            self.grow();
        }
        while layer < self.layer {
            self.shrink();
        }
    }
}

fn manhattan_distance(target: (i32, i32)) -> u32 {
    let (x, y) = target;
    debug_assert_ne!(x, i32::min_value());
    debug_assert_ne!(y, i32::min_value());
    (x.abs() as u32) + (y.abs() as u32)
}

fn adjacent_coordinates(src: (i32, i32)) -> [(i32, i32); 8] {
    let (x, y) = src;
    [
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
    ]
}

struct StressedSpiral {
    entries: Vec<u32>,
    lookup: UlamSpiral,
}

impl StressedSpiral {
    fn new() -> Self {
        StressedSpiral {
            entries: vec![0, 1],
            lookup: UlamSpiral::new(),
        }
    }
    fn last_val_big_enough(&self, target: u32) -> bool {
        self.entries.last().unwrap() > &target
    }

    fn grow(&mut self) {
        let new_index = self.entries.len() as u32;
        let new_coords = self.lookup.find_coordinates(new_index);
        let adj_coords = adjacent_coordinates(new_coords);
        let adj_indices: Vec<u32> = adj_coords.iter().map(|c| self.lookup.find_index(*c)).collect();
        let new_entry = adj_indices.into_iter().flat_map(|i| self.entries.get(i as usize)).sum();
        self.entries.push(new_entry);
    }

    fn last(&self) -> u32 {
        *self.entries.last().unwrap()
    }
}

#[test]
fn index_loc_test() {
    let mut spiral = UlamSpiral::new();
    assert_eq!(spiral.find_index((3, -1)), 27);
}

#[test]
fn index_loc_test2() {
    let mut spiral = UlamSpiral::new();
    assert_eq!(spiral.find_index((-3, 2)), 38);
}

#[test]
fn index_loc_test3() {
    let mut spiral = UlamSpiral::new();
    assert_eq!(spiral.find_index((3, -3)), 49);
}