use std::fs;

type List = Vec<String>;

fn main() {
    let input = get_input("day4/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
}

fn part1(l: &List) -> u64 {
    let mut result = 0;
    for i in 0..l.len() {
        let mut pos = 0;
        while let Some(p) = l[i][pos..].find('@') {
            pos += p;
            if get_adj(l, i, pos) < 5 {result += 1}
            pos += 1;
        }
    }
    result
}

fn part2(mut l: List) -> u64 {
    let mut result = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for i in 0..l.len() {
            let mut pos = 0;
            while let Some(p) = l[i][pos..].find('@') {
                pos += p;
                if get_adj(&l, i, pos) < 5 {
                    result += 1;
                    l[i].replace_range(pos..pos+1, ".");
                    removed = true;
                }
                pos += 1;
            }
        }
    }
    result
}

fn get_adj (l: &List, i: usize, pos: usize) -> usize {
    let mut adj = "".to_string();
    let a = if pos == 0 {0} else {pos-1};
    let b = if pos == l[i].len()-1 {l[i].len()} else {pos+2};

    if i >= 1 {adj += &l[i-1][a..b]}
    adj += &l[i][a..b];
    if i <= l.len() - 2 {adj += &l[i+1][a..b]}

    adj.chars().filter(|c|*c == '@').count()
}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.chars()
            .collect()
        })
        .collect()
}
