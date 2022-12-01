use std::collections::BinaryHeap;

pub fn solve_a(input: &str) -> i32 {
    calories(input).pop().unwrap()
}

pub fn solve_b(input: &str) -> i32 {
    alternative_calories(input).into_iter().take(3).sum()
}

fn calories(input: &str) -> BinaryHeap<i32> {
    // A proper readable and efficient solution.

    let mut heap = BinaryHeap::new();

    let mut cur_elf = 0;
    for line in input.split('\n') {
        match str::parse::<i32>(line) {
            Ok(val) => cur_elf += val,
            Err(_) => {
                heap.push(cur_elf);
                cur_elf = 0;
            }
        }
    }

    heap
}

fn alternative_calories(input: &str) -> impl IntoIterator<Item = i32> {
    // A clever but unreadable solution.

    let mut res: Vec<i32> = input
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split('\n')
                .map(|inventory| inventory.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();

    res.sort_by(|a, b| b.cmp(a));
    res
}
