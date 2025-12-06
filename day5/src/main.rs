use std::fs;

type List = Vec<u64>;

fn main() {
    let (ranges, ids) = get_input("day5/input.txt");
    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Part 2: {}", part2(&ranges));
}

fn part1(ranges: &[List], ids: &List) -> usize {
    ids.iter().filter(|i| {
        for j in ranges.iter() {
            if (j[0]..=j[1]).contains(i) {
                return true
            }
        }
        false
    }).count()
}

fn part2(ranges: &[List]) -> u64 {
    let mut new_ranges: Vec<List> = vec![];
    for range in ranges.iter() {
        let mut add_range = true;
        let mut remove_ranges = vec![];
        let mut i = [range[0], range[1]].to_vec();
        for (n, j) in new_ranges.iter().enumerate() {
            let range_i = i[0]..=i[1];
            let range_j = j[0]..=j[1];
            if range_j.contains(&i[0]) {
                if range_j.contains(&i[1]) {
                    add_range = false;
                    continue
                }
                i = [j[1]+1, i[1]].to_vec();
            } else if range_i.contains(&j[0]) {
                if !range_i.contains(&j[1]) {
                    i = [i[0], j[1]].to_vec();
                }
                remove_ranges.push(n);
            }
        }
        for n in remove_ranges.iter() {new_ranges.swap_remove(*n);}
        if add_range {new_ranges.push(i)}
    }
    new_ranges.iter().map(|r|r[1]+1 - r[0]).sum()
}

fn get_input(filename: &str) -> (Vec<List>, List) {
    let mut ranges = vec![];
    let mut ids = vec![];
    let file = fs::read_to_string(filename).expect("Should have read file");
    let lines = file.lines();
    let mut parse_ids = false;
    for line in lines{
        if parse_ids {
            ids.push(line.parse::<u64>().expect("{n} is not an integer"));
            continue
        }
        if line.is_empty() {
            parse_ids = true;
            continue
        }
        ranges.push(line.split("-").map(|n| n.parse::<u64>().expect("{n} is not an integer")).collect());
    }

    (ranges, ids)
}
