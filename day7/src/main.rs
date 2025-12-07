use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

type List = Vec<String>;

fn main() {
    let input = get_input("day7/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &List) -> u64 {
    let mut result = 0;
    let mut pos_vec = HashSet::from([input[0].find("S").unwrap()]);
    for i in (2..input.len()).step_by(2) {
        let mut temp_vec = HashSet::new();
        for p in pos_vec {
            if input[i].chars().nth(p) == Some('^') {
                result += 1;
                temp_vec.insert(p-1);
                temp_vec.insert(p+1);
            } else {
                temp_vec.insert(p);
            }
        }
        pos_vec = temp_vec;
    }
    result
}

fn part2(input: &List) -> u64 {
    let mut split_map = HashMap::from([(input[0].find("S").unwrap(), 1)]);
    for i in (2..input.len()).step_by(2) {
        let mut temp_map = HashMap::new();
        for (p, split_n) in split_map {
            if input[i].chars().nth(p) == Some('^') {
                *temp_map.entry(p-1).or_default() += split_n;
                *temp_map.entry(p+1).or_default() += split_n;
            } else {
                *temp_map.entry(p).or_default() += split_n;
            }
        }
        split_map = temp_map;
    }
    split_map.values().sum::<u64>()
}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l|l.to_string())
        .collect::<List>()
}
