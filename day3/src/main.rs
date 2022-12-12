use itertools::Itertools;
use std::collections::BTreeSet;
use std::include_str;

fn main() {
    let input1 = include_str!("../input_part_1");
    let part1 = elf_mess_cost(input1);
    println!("{part1}");
    assert_eq!(part1, 7742);

    let part2 = badges(input1);
    println!("{part2}");
    assert_eq!(part2, 2276);
}

fn badges(s: &str) -> u32 {
    s.trim()
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let (one, two, three) = (
                group.next().unwrap(),
                group.next().unwrap(),
                group.next().unwrap(),
            );
            let common = common_chars(one, two);
            let common = common_chars(&common, three);
            priority(common.chars().next().unwrap())
        })
        .sum()
}

fn elf_mess_cost(s: &str) -> u32 {
    s.trim().split('\n').map(common_item).map(priority).sum()
}

fn common_chars(a: &str, b: &str) -> String {
    let mut first = BTreeSet::new();
    let mut second = BTreeSet::new();

    for c in a.chars() {
        first.insert(c);
    }

    for c in b.chars() {
        second.insert(c);
    }

    first.intersection(&second).cloned().collect()
}

fn common_item(s: &str) -> char {
    let front: String = s.chars().take(s.len() / 2).collect::<String>();
    let back: String = s.chars().rev().take(s.len() / 2).collect::<String>();
    common_chars(&front, &back).chars().next().unwrap()
}

fn priority(c: char) -> u32 {
    let o: u32 = c.into();
    match o {
        97..=123 => o - 96,
        65..=90 => o - 38,
        _ => 0,
    }
}
