use std::collections::HashSet;
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut detector = MemoryBankCycleDetector::from_str(input).unwrap();
    detector.run_until_cycle()
}

fn part2(input: &str) -> u32 {
    let mut detector = TaggedMemoryBankCycleDetector::from_str(input).unwrap();
    detector.run_until_cycle()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct MemoryBank([u8; 16]);

impl MemoryBank {
    fn redistribute_blocks(&mut self) {
        let mut index = self.0.iter().enumerate().rev().max_by_key(|&(_, v)| *v).unwrap().0;
        let mut blocks = self.0[index];
        self.0[index] = 0;
        while blocks > 0 {
            index += 1;
            self.0[index % 16] += 1;
            blocks -= 1;
        }
    }
}

impl FromStr for MemoryBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bank = [0u8; 16];
        for (b, v) in bank.iter_mut().zip(s.split_whitespace().map(u8::from_str)) {
            *b = match v {
                Ok(u) => u,
                Err(_) => return Err(()),
            }
        }

        Ok(MemoryBank(bank))
    }
}

struct MemoryBankCycleDetector {
    bank: MemoryBank,
    set: HashSet<MemoryBank>,
}

impl FromStr for MemoryBankCycleDetector {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bank = MemoryBank::from_str(s)?;
        let set = HashSet::new();
        Ok(MemoryBankCycleDetector{bank, set})
    }
}


impl MemoryBankCycleDetector {
    fn run_until_cycle(&mut self) -> u32 {
        while self.set.insert(self.bank) {
            self.bank.redistribute_blocks()
        }
        self.set.len() as u32
    }
}

#[derive(Copy, Clone)]
struct TaggedMemoryBank {
    gen: u32,
    bank: MemoryBank,
}

impl TaggedMemoryBank {
    fn new(gen: u32, bank: MemoryBank) -> Self {
        TaggedMemoryBank{gen, bank}
    }
}

impl Hash for TaggedMemoryBank {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bank.hash(state);
    }
}

impl PartialEq for TaggedMemoryBank {
    fn eq(&self, other: &TaggedMemoryBank) -> bool {
        self.bank == other.bank
    }
}

impl Eq for TaggedMemoryBank {}


struct TaggedMemoryBankCycleDetector {
    gen: u32,
    bank: MemoryBank,
    set: HashSet<TaggedMemoryBank>,
}

impl TaggedMemoryBankCycleDetector {
    fn run_until_cycle(&mut self) -> u32 {
        loop {
            let tagged = TaggedMemoryBank::new(self.gen, self.bank);
            match self.set.replace(tagged) {
                None => {
                    self.gen += 1;
                    self.bank.redistribute_blocks();
                },
                Some(t) => return self.gen - t.gen
            }
        }
    }
}

impl FromStr for TaggedMemoryBankCycleDetector {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bank = MemoryBank::from_str(s)?;
        let set = HashSet::new();
        let gen = 0;
        Ok(TaggedMemoryBankCycleDetector{gen, bank, set})
    }
}