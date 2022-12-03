use std::collections::HashSet;

use itertools::Itertools;

pub fn solve_a(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let (p1, p2) = line.split_at(line.len() / 2);

            let set1: HashSet<char> = HashSet::from_iter(p1.chars());
            let set2 = HashSet::from_iter(p2.chars());

            set1.intersection(&set2).map(|c| priority(*c)).sum::<i32>()
        })
        .sum()
}

pub fn solve_b(input: &str) -> i32 {
    input
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|elf| HashSet::from_iter(elf.chars()))
                .reduce(|acc: HashSet<char>, elf| &acc & &elf)
                .unwrap_or_default()
                .into_iter()
                .map(priority)
                .sum::<i32>()
        })
        .sum()
}

fn priority(c: char) -> i32 {
    (if c.is_ascii_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        u32::from(c) - u32::from('A') + 27
    })
    .try_into()
    .unwrap()
}
