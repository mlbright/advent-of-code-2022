use std::include_str;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", score(INPUT));
}

fn score(s: &str) -> usize {
    s.trim().split('\n').map(battle).sum()
}

fn battle(l: &str) -> usize {
    let mut line = l.splitn(2, ' ').fuse();
    let opponent = line.next().unwrap();
    let me = line.next().unwrap();
    let standoff = match (
        opponent.chars().take(1).next().unwrap(),
        me.chars().take(1).next().unwrap(),
    ) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => 0,
    };
    let shape = match me.chars().take(1).next().unwrap() {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };
    standoff + shape
}
