use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|report| report.map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter()
        .map(|report| is_linear(report))
        .filter(|&x| x)
        .count() as i32
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter()
        .map(|report| is_linear_with_dampener(report))
        .filter(|&x| x)
        .count() as i32
}

fn is_linear(line: &Vec<i32>) -> bool {
    let inc = (line[1] - line[0]) > 0;
    if inc {
        // it its incremental, then all values must be incremental
        line.iter().zip(line.iter().skip(1)).all(|(a, b)| a < b && b - a > 0 && b - a <= 3)
    } else {
        // if its decremental, then all values must be decremental
        line.iter().zip(line.iter().skip(1)).all(|(a, b)| a > b && a - b > 0 && a - b <= 3)
    }
}

fn is_linear_with_dampener(line: &Vec<i32>) -> bool {
    is_linear(line) || (0..line.len())
        .map(|i| {
            let mut modified = line.clone();
            modified.remove(i);
            modified
        })
        .any(|modified| is_linear(&modified))
}
