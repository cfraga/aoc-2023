use std::{fs::File, io::{BufReader, BufRead}};

use log::debug;
use regex::{Regex, Captures};


pub fn part1(file_path: String) -> u32 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    
    reader.lines()
    .flat_map( |maybe_l| maybe_l.ok())
    .map(|l| first_last_digits(l))
    .sum()
}

pub fn part2(file_path: String) -> u32 {
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);

    reader.lines()
        .flat_map( |maybe_l| maybe_l.ok())
        .map( |l| literal_to_number(l))
        .map(|l| first_last_digits(l))
        .sum()
}

fn first_last_digits(line: String) -> u32 {
    let linedigits = line.chars().filter( |c| matches!(c, '0'..='9')).collect::<String>();
    
    let concatted = format!("{}{}",linedigits.chars().nth(0).unwrap(),linedigits.chars().last().unwrap()).parse::<u32>().unwrap();
    debug!("\ndigits:{} - line: {}", concatted, linedigits);

    concatted
}

fn literal_to_number(line:String) -> String {
    debug!("{}", line);
    // feel dirty instancing the regex for every line
    let re = Regex::new("(oneight|fiveight|threeight|twone|sevenine|eighthree|nineight|eightwo|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    re.replace_all(&line, |cap: &Captures | {
        match &cap[0] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            "oneight" => "18",
            "fiveight" => "58",
            "twone" => "21",
            "sevenine" => "79",
            "eighthree" => "83",
            "nineight" => "98",
            "eightwo" => "82",
            "threeight" => "38",
            _ => panic!("huh, smth wrong?")
        }
    }).to_string()
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = "src/day1/sample_1".to_string();
        let expected = 142;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = "src/day1/input".to_string();
        let expected: u32 = 55130;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = "src/day1/sample_2".to_string();
        let expected = 281;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = "src/day1/input".to_string();
        let expected: u32 = 54985;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

}