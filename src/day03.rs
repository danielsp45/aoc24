use aoc_runner_derive::aoc;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Command {
    Mul(i32, i32),
    Do, 
    Dont,
}

impl Command {
    fn parse(input: &str) -> Vec<Command> {
        let re = Regex::new(r"mul(\((\d\d?\d?)\,(\d\d?\d?)\))|do\(\)|don't\(\)").unwrap();

        let mut vec = Vec::new();
        for cap in re.captures_iter(input) {
            if cap[0] == *"do()" {
                vec.push(Command::Do);
            } else if cap[0] == *"don't()" {
                vec.push(Command::Dont);
            } else {
                let x = cap[2].parse::<i32>().unwrap();
                let y = cap[3].parse::<i32>().unwrap();
                vec.push(Command::Mul(x, y));
            }
        }

        vec
    }

    fn execute(&self) -> i32 {
        match self {
            Command::Mul(x, y) => x * y,
            _ => 0,
        }
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let commands = Command::parse(input);
    let mut result = 0;
    for command in commands.iter() {
        result += command.execute();
    }

    result
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let commands = Command::parse(input);

    let mut result = 0;
    let mut state = true;
    for command in commands {
        if command == Command::Do {
            state = true;
        } else if command == Command::Dont {
            state = false;
        } else if state {
            result += command.execute();
        }
    }

    result
}
