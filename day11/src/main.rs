use std::fs;
use std::collections::HashMap;

type List = Vec<String>;

fn main() {
    let input = get_input("day11/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(device_map: &HashMap<String, List>) -> usize {
    get_paths("you", "out", device_map, &mut HashMap::new())
}

fn part2(device_map: &HashMap<String, List>) -> usize {
    get_paths("svr", "dac", device_map, &mut HashMap::new()) *
    get_paths("dac", "fft", device_map, &mut HashMap::new()) *
    get_paths("fft", "out", device_map, &mut HashMap::new()) +
    get_paths("svr", "fft", device_map, &mut HashMap::new()) *
    get_paths("fft", "dac", device_map, &mut HashMap::new()) *
    get_paths("dac", "out", device_map, &mut HashMap::new())
}

fn get_paths(d: &str, end: &str, d_map: &HashMap<String, List>, p_map: &mut HashMap<String, usize>) -> usize {
    if d == end {return 1}
    if p_map.contains_key(d) {return p_map[d]}
    let total = match d_map.get(d) {
        Some(_) => d_map[d].iter().map(|o| get_paths(o, end, d_map, p_map)).sum(),
        None => 0
    };
    p_map.insert(d.to_string(), total);
    total
}

fn get_input(filename: &str) -> HashMap<String, List> {
    let list = fs::read_to_string(filename).expect("Should have read file")
                                .lines()
                                .map(|l|l.to_string())
                                .collect::<List>();
    let mut map = HashMap::new();
    for l in list {
        let entry = l.split(": ").map(|v| v.to_string()).collect::<List>();
        map.insert(entry[0].clone(), entry[1].split(" ").map(|v| v.to_string()).collect());
    }
    map
}
