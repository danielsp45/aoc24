use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul(\((\d\d?\d?)\,(\d\d?\d?)\))").unwrap();
    let mut result = 0;
    for cap in re.captures_iter(input) {
        let x = cap[2].parse::<i32>().unwrap();
        let y = cap[3].parse::<i32>().unwrap();

        result += x * y;
    }

    result
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul(\((\d\d?\d?)\,(\d\d?\d?)\))|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut state = true;
    for cap in re.captures_iter(input) {
        if cap [0] == *"do()" {
            state = true;
        } else if cap[0] == *"don't()" {
            state = false;
        } else if state {
            let x = cap[2].parse::<i32>().unwrap();
            let y = cap[3].parse::<i32>().unwrap();

            result += x * y;
        }
    }

    result
}
