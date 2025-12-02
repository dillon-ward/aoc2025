use std::fs;
use fancy_regex::Regex;

type Range = (u64, u64);

fn main() {
    let input = get_input("day2/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[Range]) -> u64 {
    let mut total = 0;
    for i in input.iter() {
        for n in i.0..i.1 + 1 {
            let digits = n.to_string();
            let size = digits.len();
            if size % 2 == 0 && digits[..size/2] == digits[size/2..] {total += n}
        }
    }
    total
}

fn part2(input: &[Range]) -> u64 {
    let mut total = 0;
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    for i in input.iter() {
        for n in i.0..i.1 + 1 {
            let digits = n.to_string();
            if re.is_match(&digits).unwrap() {total += n}
        }
    }
    total
}

fn get_input(filename: &str) -> Vec<Range> {
    fs::read_to_string(filename).expect("Should have read file")
        .split(',')
        .map(|l| {
            match l.find('-') {
                Some(p) => {
                    (l[..p].parse::<u64>().expect("{n} is not an integer"),
                    l[p+1..].parse::<u64>().expect("{n} is not an integer"))
                }
                None => {panic!()}
            }
        })
        .collect()
}