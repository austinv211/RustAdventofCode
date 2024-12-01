use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

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
    fn box_from_string(val: &str) -> Box<ParseError> {
        Box::new(ParseError(val.into()))
    }

    fn box_from_num(num: &str) -> Box<ParseError> {
        Box::new(ParseError(format!("Couldn't parse into number: {num}")))
    }
}

// Implement a config for command line arguments
pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Filepath Missing from Arguments!");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

// The general run function for our program, accepting a config and getting the absolute distance and similarity
// between two same-sized sorrted lists of numbers using a text file as input
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Reading input from: {:?}", config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Could not read input file!");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    let mut frequency: HashMap<u32, u32> = HashMap::new();

    for line in contents.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();

        if nums.len() < 2 {
            return Err(ParseError::box_from_string(
                "Invalid number of columns in input text",
            ));
        }

        let left_parsed: u32 = match nums[0].trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => return Err(ParseError::box_from_num(nums[0])),
        };

        let right_parsed: u32 = match nums[1].trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => return Err(ParseError::box_from_num(nums[0])),
        };

        left_list.push(left_parsed);
        right_list.push(right_parsed);

        if frequency.contains_key(&right_parsed) {
            frequency.insert(right_parsed, frequency[&right_parsed] + 1);
        } else {
            frequency.insert(right_parsed, 1);
        }
    }

    if left_list.len() != right_list.len() {
        return Err(ParseError::box_from_string(
            "Lists are not the same length in input text!",
        ));
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    let mut total_similarity = 0;

    for index in 0..left_list.len() {
        total_distance += left_list[index].abs_diff(right_list[index]);

        if frequency.contains_key(&left_list[index]) {
            total_similarity += left_list[index] * frequency[&left_list[index]]
        }
    }

    println!("Total Distance: {total_distance}");
    println!("Total Similarity: {total_similarity}");

    Ok(())
}
