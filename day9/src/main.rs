use std::fs;
use itertools::Itertools;

type Pos = Vec<i64>;

fn main() {
    let input = get_input("day9/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[Pos]) -> u64 {
    let mut largest_area = 0;
    for i in 0..input.len()-1 {
        for j in i+1..input.len() {
            let area = get_area(&input[i], &input[j]);
            if area > largest_area {largest_area = area}
        }
    }
    largest_area
}

fn part2(input: &[Pos]) -> u64 {
    let lines = input
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&Pos, &Pos)>>();
    let max_box = input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, get_area(a, b)))
        .sorted_by_key(|(_, _, area)| *area)
        .rev()
        .find(|(a, b, _)| {
            lines.iter().all(|(start, end)| {
                let left = a[0].max(b[0]) <= start[0].min(end[0]);
                let right = a[0].min(b[0]) >= start[0].max(end[0]);
                let up = a[1].max(b[1]) <= start[1].min(end[1]);
                let down = a[1].min(b[1]) >= start[1].max(end[1]);
                left || right || up || down
            })
        });
    max_box.unwrap().2
}

fn get_area(p1: &Pos, p2: &Pos) -> u64 {
    (((p1[0]-p2[0]).abs() + 1) * ((p1[1]-p2[1]).abs() + 1)) as u64
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
