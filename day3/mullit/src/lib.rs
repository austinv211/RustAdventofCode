use std::{env, error::Error, fs};

// Implement a config for command line arguments
struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Filepath Missing from Arguments!");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub struct Runner {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRING: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn can_multiply() {
        assert_eq!(Some(6), multiply("2,3)moretexthere"))
    }

    #[test]
    fn handle_incorrect() {
        assert_eq!(None, multiply("2, 3)moretexthere"))
    }

    #[test]
    fn example_part1() {
        assert_eq!(161usize, Runner::part1(EXAMPLE_STRING))
    }

    #[test]
    fn example_part2() {
        assert_eq!(48usize, Runner::part2(EXAMPLE_STRING))
    }
}

fn multiply(input_str: &str) -> Option<usize> {
    let Some((left, rightwithend)) = input_str.split_once(',') else {
        return None;
    };

    let Ok(leftnum) = left.parse::<usize>() else {
        return None;
    };

    let Some((right, _)) = rightwithend.split_once(')') else {
        return None;
    };

    let Ok(rightnum) = right.parse::<usize>() else {
        return None;
    };

    if leftnum >= 1000 || rightnum >= 1000 {
        return None;
    } else {
        return Some(leftnum * rightnum);
    }
}

fn next_operation(contents: &str) -> Option<usize> {
    let mul_find = contents.find("mul(");
    let do_find = contents.find("do()");
    let dont_find = contents.find("don't(");

    [mul_find, do_find, dont_find]
        .iter()
        .filter_map(|location| *location)
        .min()
}

impl Runner {
    fn part1(contents: &str) -> usize {
        let mut position = 0;
        let mut sum = 0;
        while let Some(location) = &contents[position..].find("mul(") {
            position += location + 4;
            let Some(val) = multiply(&contents[position..]) else {
                continue;
            };
            sum += val;
        }

        sum
    }

    fn part2(contents: &str) -> usize {
        let mut position = 0;
        let mut sum = 0;
        let mut enabled = true;
        while let Some(location) = next_operation(&contents[position..]) {
            position += location;
            match &contents[position..position + 3] {
                "mul" => {
                    if enabled {
                        if let Some(val) = multiply(&contents[position + 4..]) {
                            sum += val;
                        }
                    }
                }
                "do(" => enabled = true,
                "don" => enabled = false,
                _ => panic!("OOPS, bad pattern, {:?}", &contents[position..position + 3]),
            }
            position += 4;
        }

        sum
    }

    pub fn run() -> Result<(), Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        let Ok(config) = Config::build(&args) else {
            return Err("Incorrect commandline arguments")?;
        };
        let contents = fs::read_to_string(config.file_path).expect("Could not read input file!");

        let part1val = Self::part1(&contents);
        println!("Part 1: {part1val}");

        let part2val = Self::part2(&contents);
        println!("Part 2: {part2val}");

        Ok(())
    }
}
