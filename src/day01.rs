use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .unzip()
}

#[aoc(day1, part1)]
pub fn part1((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left = left.clone();
    let mut right = right.clone();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let hashmap = right.iter().fold(HashMap::new(), |mut acc, &value| {
        *acc.entry(value).or_insert(0) += 1;
        acc
    });

    left
        .iter()
        .map(|value| value * hashmap.get(value).unwrap_or(&0))
        .sum()
}
