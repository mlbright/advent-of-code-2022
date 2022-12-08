use itertools::Itertools;
use std::include_str;

fn main() {
    let input: &str = include_str!("input");
    println!("part 1: {}", elf_calories(input).max().unwrap_or(0));
    println!(
        "part 2: {}",
        elf_calories(input).sorted().rev().take(3).sum::<usize>()
    );
}

fn elf_calories(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.trim().split("\n\n").map(|elf| {
        elf.split('\n')
            .map(|calories| calories.parse::<usize>().unwrap_or(0))
            .sum()
    })
}
