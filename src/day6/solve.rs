use std::{fs::File, io::{BufReader, BufRead}};
use array_tool::vec::Intersect;
use log::debug;
use regex::{Regex, Captures};

const DAY: &str="day6";

pub fn part1(file_path: String) -> u64 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let lines = reader.lines().flat_map( |maybe_l| maybe_l.ok()).collect::<Vec<String>>();
    let times = lines[0].split(":").last().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let records = lines[1].split(":").last().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    
     (0..times.len())
        .map(|i| winning_odds(times[i], records[i]))
        .fold(1, |acc, n| acc * (n as u64))
}

fn winning_odds(time: u32, record: u32) -> u32 {
    let mut acc = 0;
    let mut times_max_val = 1;
    let mut prev_val = 0;
    for i in 1..time {
        let dist = i * (time - i);
        println!("record: {} dist: {} acc: {}, max:{}", record, dist, acc, times_max_val);
        if dist < prev_val { // means we've reached highest possibility
            break;
        }

        if dist == prev_val {
            times_max_val +=1;
        }

        if dist > record {
            acc +=1;
            prev_val = dist;
        }
    }


    ((acc - times_max_val) * 2 + times_max_val)
}

pub fn part2(file_path: String) -> u64 {
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    todo!();
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = format!("src/{}/sample_1", DAY).to_string();
        let expected = 288;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: u64 = 2065338;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = format!("src/{}/sample_1", DAY).to_string();
        let expected = 46;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: u64 = 52510809;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}