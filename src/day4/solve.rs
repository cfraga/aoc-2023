use std::{fs::File, io::{BufReader, BufRead}};
use array_tool::vec::Intersect;
use log::debug;
use regex::{Regex, Captures};


pub fn part1(file_path: String) -> u32 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);

    reader.lines()
        .flat_map(|maybe_l| maybe_l.ok())
        .map(|l| parse_line(l))
        .map(|(l, r)| winning_numbers(&l, &r))
        .filter(|n| *n > 0)
        .map(|n| score(n))
        .inspect(|n| println!("score: {}", n))
        .sum()
}

// sample Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_line(line: String) -> (Vec<u8>, Vec<u8>) {

    let res:Vec<Vec<u8>> = line
        .split(": ")
        .skip(1) //ignoring card number/name
        .flat_map( |rest| rest.split(" | "))
        .map(|numbers| numbers.split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect::<Vec<u8>>())
        .collect();

    (res[0].clone(), res[1].clone())
}

// pub fn part2(file_path: String) -> u32 {
//     let f = File::open(file_path).expect("couldnt open file");
//     let reader = BufReader::new(f);
//     let mut numbers = Vec::new();
//     let mut parts = Vec::new();
    
//     for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
//         let (n, p) = parse_line(l, n);
//         numbers.extend(n);
//         parts.extend(p);
//     }
   
//     calculate_gear_ratios(numbers, parts)
// }


fn winning_numbers(left: &Vec<u8>, right: &Vec<u8>) -> u8 {
    left.intersect(right.clone()).len() as u8
}

fn score(n: u8) -> u32 {
    let base: u32 = 2;

    base.pow((n-1).into())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = "src/day4/sample_1".to_string();
        let expected = 13;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = "src/day4/input".to_string();
        let expected: u32 = 2;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    // #[test]
    // pub fn test_sample_2(){
    //     let test_file = "src/day3/sample_1".to_string();
    //     let expected = 467835;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }

    // #[test]
    // pub fn test_input_2(){
    //     let test_file = "src/day3/input".to_string();
    //     let expected: u32 = 75519888;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }
}