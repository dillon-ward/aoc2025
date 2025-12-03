use std::fs;

type Bank = Vec<u8>;

fn main() {
    let input = get_input("day3/input.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[Bank]) -> u64 {
    input.iter().map(|b|get_joltage(b, 2)).sum()
}

fn part2(input: &[Bank]) -> u64 {
    input.iter().map(|b|get_joltage(b, 12)).sum()
}

fn get_joltage(bank: &Bank, len: u8) -> u64 {
    let mut b = bank.clone();
    let mut joltage: String = "".to_string();
    for i in 0..len {
        if ((len-i) as usize) >= b.len() {
            let rest: String = b.iter().map(|n|n.to_string()).collect();
            joltage += &rest;
            break;
        }
        let temp =  b[..b.len()-(len-i-1) as usize].to_vec();
        let d = temp.iter().max().unwrap();
        b = b[b.iter().position(|n|n == d).unwrap()+1..].to_vec();
        joltage += &d.to_string()
    }
    joltage.parse::<u64>().unwrap()
}

fn get_input(filename: &str) -> Vec<Bank> {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| l.chars().map(|c |c.to_string().parse::<u8>().unwrap()).collect())
        .collect()
}
