use std::include_str;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", score(INPUT, battle1));
    println!("{}", score(INPUT, battle2));
}

fn score(s: &str, f: fn(char, char) -> usize) -> usize {
    s.trim().split('\n').map(|t| battle_harness(t, f)).sum()
}

fn battle_harness(s: &str, f: fn(char, char) -> usize) -> usize {
    let mut line = s.splitn(2, ' ').fuse();
    let opponent = line.next().unwrap().chars().take(1).next().unwrap();
    let me = line.next().unwrap().chars().take(1).next().unwrap();
    f(opponent, me)
}

fn battle2(opponent: char, me: char) -> usize {
    match me {
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

fn battle1(opponent: char, me: char) -> usize {
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
