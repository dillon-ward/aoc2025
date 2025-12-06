use std::fs;

type List = Vec<String>;
type Nums = Vec<u64>;

fn main() {
    let input = get_input("day6/input.txt");
    let ops = split_str(&input[input.len()-1]);
    println!("Part 1: {}", part1(&input[..input.len()-1], &ops));
    println!("Part 2: {}", part2(&input[..input.len()-1], &ops));
}

fn part1(input: &[String], ops: &[String]) -> u64 {
    let mut result = 0;
    let nums_vec = get_nums_vec(&input.iter().map(|s|split_str(s)).collect::<Vec<List>>());
    for (i, nums) in nums_vec.iter().enumerate() {
        if ops[i] == "+" {result += nums.iter().sum::<u64>()} else {result += nums.iter().product::<u64>()}
    }
    result
}

fn part2(input: &[String], ops: &[String]) -> u64 {
    let mut result = 0;
    let mut pos = 0;
    for i in 0..ops.len() {
        let mut nums: Vec<String> = vec![];
        if i == ops.len() - 1 {
            for s in input.iter() {nums.push(s[pos..].to_string())}
            result += get_ceph_calc(&nums, &ops[i]);
            break
        }
        let mut lengths: Vec<usize> = vec![];
        for s in input.iter() {lengths.push(s[pos..].find(" ").unwrap())}
        let slice_pos = pos + lengths.iter().max().unwrap();
        for s in input.iter() {nums.push(s[pos..slice_pos].to_string())}
        result += get_ceph_calc(&nums, &ops[i]);
        pos = slice_pos + 1;
    }
    result
}

fn get_nums_vec(input: &[List]) -> Vec<Nums> {
    let mut nums_vec = vec![];
    for i in 0..input[0].len() {
        let mut nums = vec![];
        for ns in input.iter() {
            nums.push(ns[i].parse::<u64>().expect("{n} is not an integer"));
        }
        nums_vec.push(nums);
    }
    nums_vec
}

fn get_ceph_calc(nums: &[String], op: &str) -> u64 {
    let mut ceph_nums = vec![];
    for j in 0..nums[0].len() {
        let mut ceph_num = "".to_string();
        for n in nums {if &n[j..j+1] != " " {ceph_num += &n[j..j+1]}}
                ceph_nums.push(ceph_num.parse::<u64>().unwrap())
    }
    if op == "+" {ceph_nums.iter().sum::<u64>()} else {ceph_nums.iter().product::<u64>()}
}

fn split_str(str: &str) -> List {
    str.split_ascii_whitespace()
        .map(|s|s.to_string())
        .collect()
}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l|l.to_string())
        .collect::<List>()
}
