use std::fs;
use itertools::Itertools;

type List = Vec<String>;
type Nums = Vec<f64>;

const EPSILON: f64 = 1e-9;

fn main() {
    let input = get_input("day10/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[List]) -> usize {
    input.iter().map(|machine| {

        let goal = machine[0].trim_matches(['[', ']']);
        let buttons = get_buttons(&machine[1..machine.len()-1]);
        let button_len = buttons.len();
        let start = ".".repeat(goal.len());

        for i in 1..button_len {
            for buttons in buttons.iter().combinations(i) {
                let mut cur_lights = start.clone();
                for button in buttons {cur_lights = switch_lights(&cur_lights, button)}
                if cur_lights == goal {return i}
            }
        }

        button_len
    })
    .sum()
}

fn part2(input: &[List]) -> usize {
    input.iter().map(|machine| {

        let buttons = get_buttons(&machine[1..machine.len()-1]);
        let joltages = get_joltages(&machine[machine.len()-1]);

        let matrix = Matrix::echelon_form(&buttons, &joltages);
        let max = joltages.iter().max().unwrap() + 1;
        let mut min = usize::MAX;
        let mut values = vec![0; matrix.independents.len()];

        dfs(&matrix, 0, &mut values, &mut min, max);

        min
    })
    .sum()
}

struct Matrix {
    data: Vec<Nums>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>
}

impl Matrix {
    // Create a matrix in echelon form by performing Gaussian elimination
    fn echelon_form(buttons: &[Vec<usize>], joltages: &[usize]) -> Self {
        let rows = joltages.len();
        let cols = buttons.len();
        let mut data = vec![vec![0_f64; cols + 1]; rows];

        // Add buttons column-wise
        for (c, button) in buttons.iter().enumerate() {
            for &r in button {
                if r < rows {data[r][c] = 1.0}
            }
        }

        // Add joltages to last column
        for (r, &val) in joltages.iter().enumerate() {data[r][cols] = val as f64}

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new()
        };

        matrix.gauss_elim();
        matrix
    }

    fn gauss_elim(&mut self) {
        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.cols {

            // Find the best pivot row for the current column
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable
            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue
            }

            // Otherwise, swap rows and mark column as dependent
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalise pivot row
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {*val /= pivot_value}

            // Eliminate this column in all other rows
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| *val -= factor * pivot_val);
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);
    }

    // Check if the values of the independent variables are valid. If so, return total presses
    fn valid(&self, values: &[usize]) -> Option<usize> {

        // Get how many times the free variables have been pressed
        let mut total = values.iter().sum();

        // Calculate dependent variable values based on independent variables
        for row in 0..self.dependents.len() {
            
            // Subtract the sum of the free variable presses from the solution
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // Ensure values are positive and whole for a valid solution
            if val < -EPSILON {return None}

            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {return None}

            total += rounded as usize
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, i: usize, values: &mut [usize], min: &mut usize, max: usize) {

    // Check the solution is valid once all independent variables have been assigned
    if i == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {*min = (*min).min(total)}
        return
    }

    // Try different values for the current independent variable
    let total: usize = values[..i].iter().sum();
    for val in 0..max {
        if total + val >= *min {break}
        values[i] = val;
        dfs(matrix, i + 1, values, min, max);
    }
}

fn get_buttons(l: &[String]) -> Vec<Vec<usize>> {
    l
        .iter()
        .map(|b| {
            b.trim_matches(['(', ')'])
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_joltages(l: &str) -> Vec<usize> {
    l
        .trim_matches(['{', '}'])
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect()
}

fn switch_lights(lights: &str, button: &[usize]) -> String {
    let mut cur_lights = lights.to_string();
    for &n in button {
        if cur_lights[n..n+1] == *"." {
            cur_lights.replace_range(n..n+1, "#");
        } else {
            cur_lights.replace_range(n..n+1, ".");
        }
    }
    cur_lights.to_string()
}

fn get_input(filename: &str) -> Vec<List> {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect()
        })
        .collect()
}
