use std::fs;

type List = Vec<String>;
type Nums = Vec<usize>;

fn main() {
    let input = get_input("day12/input.txt");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &[(Nums, Nums)]) -> usize {
    input
        .iter()
        .filter(|&v| v.0.iter().product::<usize>() >= 9 * v.1.iter().sum::<usize>())
        .count()
}

fn get_input(filename: &str) -> Vec<(Nums, Nums)>  {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            let temp = l.split(": ").map(|v| v.to_string()).collect::<List>();
            let region = temp[0].split("x").map(|v| v.parse::<usize>().unwrap()).collect();
            let presents = temp[1].split(" ").map(|v| v.parse::<usize>().unwrap()).collect();
            (region, presents)
        }).collect()
}
