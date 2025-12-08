use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = Vec<i64>;
type Dist = ((usize, usize), i64);

fn main() {
    let input = get_input("day8/input.txt");
    let dists = get_dists(&input);
    println!("Part 1: {}", part1(&dists));
    println!("Part 2: {}", part2(&dists, &input));
}

fn part1(dists: &[Dist]) -> u32 {
    let mut connections = 1000;
    let mut circuit_map = HashMap::new();
    let mut i = 0;
    let mut circuit_num = 0;
    while connections > 0 {
        let (b1, b2) = dists[i].0;
        if circuit_map.contains_key(&b1) && circuit_map.contains_key(&b2) {
            let mut same = vec![];
            for (&b, n) in circuit_map.iter() {
                if n == circuit_map.get(&b1).unwrap() {same.push(b);}
            }
            for s in same.iter() {circuit_map.insert(*s, *circuit_map.get(&b2).unwrap());}
        }
        if circuit_map.contains_key(&b1) {circuit_map.insert(b2, *circuit_map.get(&b1).unwrap());}
        else if circuit_map.contains_key(&b2) {circuit_map.insert(b1, *circuit_map.get(&b2).unwrap());}
        else {
            circuit_map.insert(b1, circuit_num);
            circuit_map.insert(b2, circuit_num);
            circuit_num += 1;
        }
        i += 1;
        connections -= 1;
    }
    let mut size_map = HashMap::new();
    for &n in circuit_map.values() {*size_map.entry(n).or_default() += 1;}
    let mut sizes: Vec<u32> = size_map.values().copied().collect();
    sizes.sort();
    sizes[sizes.len()-3..].iter().product()
}

fn part2(dists: &[Dist], input: &[Pos]) -> i64 {
    let mut box_set = HashSet::new();
    let mut i = 0;
    loop {
        let ((b1, b2), _) = dists[i];
        box_set.insert(b1);
        box_set.insert(b2);
        if box_set.len() == input.len() {
            return input[b1][0] * input[b2][0]
        }
        i += 1;
    }
}

fn get_dists(input: &[Pos]) -> Vec<Dist> {
    let mut dists = vec![];
    for i in 0..input.len()-1 {
        for j in i+1..input.len() {
            dists.push(((i,j), get_dist(&input[i], &input[j])));
        }
    }
    dists.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));
    dists
}

fn get_dist(p1: &Pos, p2: &Pos) -> i64 {
    ((p1[0]-p2[0]).pow(2) + (p1[1]-p2[1]).pow(2) + (p1[2]-p2[2]).pow(2)).isqrt()
}

fn get_input(filename: &str) -> Vec<Pos> {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.split(',').map(|n| n.parse::<i64>().expect("{n} is not an integer"))
            .collect()
        })
        .collect()
}
