use std::fs;

type List = Vec<i32>;

static START: i32 = 50;
static MOD: i32 = 100;

fn main() {
    let input = get_input("day1/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &List) -> i32 {
    let mut n = START;
    let mut count = 0;
    for i in input.iter() {
        n = (n + i) % MOD;
        if n == 0 {count += 1}
    }
    count
}

fn part2(input: &List) -> i32 {
    let mut n = START;
    let mut count = 0;
    for i in input.iter() {
        if *i < 0 && i.abs() >= n {
            count += (MOD - n - i) / MOD;
            if n == 0 {count -= 1}
        } else if *i > 0 && (n + i) >= MOD {
            count += (n + i) / MOD;
        }
        n = (n + i) % MOD;
        if n < 0 {
            n += MOD;
        }
    }
    count
}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            if l.starts_with('L') {
                -l[1..].parse::<i32>().unwrap()
            } else {
                l[1..].parse::<i32>().unwrap()
            }
        })
        .collect()
}
