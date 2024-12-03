use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

// Implement a ParseError for easy error handling
#[derive(Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing input text, {}", self.0)
    }
}

impl Error for ParseError {}

impl ParseError {
    fn box_from_num(num: &str) -> Box<ParseError> {
        Box::new(ParseError(format!("Couldn't parse into number: {num}")))
    }
}

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

impl Runner {
    fn get_safe_unsafe(parsed_nums: &Vec<u32>) -> u32 {
        fn check_abs_diff(num: u32, compare: u32) -> bool {
            let ab_diff = num.abs_diff(compare);
            ab_diff <= 3 && ab_diff >= 1
        }

        let mut result = 1; // for safe or unsafe
        let mut direction = 0; // for increase or decrease

        // do the comparisons with the previous
        for (index, num) in parsed_nums.clone().into_iter().enumerate() {
            if index > 0 {
                match num.cmp(&parsed_nums[index - 1]) {
                    Ordering::Less => {
                        // check increasing or decreasing
                        if direction != -1 && direction != 0 {
                            result = 0;
                            break;
                        } else {
                            direction = -1;
                            if !check_abs_diff(num, parsed_nums[index - 1]) {
                                result = 0;
                                break;
                            }
                        }
                    }
                    Ordering::Greater => {
                        // check increasing or decreasing
                        if direction != 1 && direction != 0 {
                            result = 0;
                        } else {
                            direction = 1;
                            if !check_abs_diff(num, parsed_nums[index - 1]) {
                                result = 0;
                                break;
                            }
                        }
                    }
                    Ordering::Equal => {
                        result = 0;
                        break;
                    }
                }
            }
        }

        return result;
    }

    // General run function for our program, accepts config info from env args and outputs question answer via print
    pub fn run() -> Result<(), Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem getting arguments: {err}");
            process::exit(1);
        });
        println!("Reading input from: {:?}", config.file_path);
        let contents = fs::read_to_string(config.file_path).expect("Could not read input file!");

        let mut safe_count = 0;
        for line in contents.lines() {
            let mut parsed_nums: Vec<u32> = Vec::new();

            // parse the line into numbers
            let split_on_white: Vec<&str> = line.split_whitespace().collect();
            for s_num in split_on_white {
                let p_num = match s_num.trim().parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => return Err(ParseError::box_from_num(s_num)),
                };
                parsed_nums.push(p_num);
            }

            let mut result = Self::get_safe_unsafe(&parsed_nums);
            if result == 0 {
                for index in 0..parsed_nums.len() {
                    let mut without_index = parsed_nums.to_vec();
                    without_index.remove(index);
                    result = Self::get_safe_unsafe(&without_index);
                    if result == 1 {
                        break;
                    }
                }
            }

            safe_count += result;
        }

        println!("{safe_count} Reports are safe.");

        Ok(())
    }
}
