use std::include_str;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", score_part_1(INPUT));
    println!("{}", score_part_2(INPUT));
}

fn score_part_2(s: &str) -> usize {
    s.trim().split('\n').map(battle2).sum()
}

fn score_part_1(s: &str) -> usize {
    s.trim().split('\n').map(battle1).sum()
}

fn battle2(l: &str) -> usize {
    let mut line = l.splitn(2, ' ').fuse();
    let opponent = line.next().unwrap().chars().take(1).next().unwrap();
    let outcome = line.next().unwrap().chars().take(1).next().unwrap();
    match outcome {
        'X' => match opponent {
            'A' => shape('Z'),
            'B' => shape('X'),
            'C' => shape('Y'),
            _ => 0,
        },
        'Y' => match opponent {
            'A' => 3 + shape('X'),
            'B' => 3 + shape('Y'),
            'C' => 3 + shape('Z'),
            _ => 0,
        },
        'Z' => match opponent {
            'A' => 6 + shape('Y'),
            'B' => 6 + shape('Z'),
            'C' => 6 + shape('X'),
            _ => 0,
        },
        _ => 0,
    }
}

fn battle1(l: &str) -> usize {
    let mut line = l.splitn(2, ' ').fuse();
    let opponent = line.next().unwrap().chars().take(1).next().unwrap();
    let me = line.next().unwrap().chars().take(1).next().unwrap();
    let standoff = match (opponent, me) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => 0,
    };
    standoff + shape(me)
}

fn shape(c: char) -> usize {
    match c {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}
