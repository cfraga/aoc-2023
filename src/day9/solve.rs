use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, num::Wrapping, sync::Arc, borrow::{BorrowMut, Borrow}};
use array_tool::vec::Intersect;

use log::debug;

const DAY: &str="day9";

#[derive(Debug, Clone)]
pub struct Node {
    val: String,
    l: String,
    r: String,
}

pub fn part1(file_path: String) -> i128 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    
    reader.lines()
        .flat_map( |maybe_l| maybe_l.ok())
        .map(|l| l.split_whitespace().map(|n| n.parse::<i128>().unwrap()).collect())
        .map(|sequence| calculate_sequence(sequence))
        .sum()
}

fn calculate_sequence(seq: Vec<i128>) -> i128 {
   let mut acc = 0;
   let mut executions = vec![];
   executions.push(seq);
   loop {
        executions.push(seq_differences(executions.last().unwrap()));

        if executions.last().unwrap().iter().all(|n| *n == 0i128) {
            break;
        }
   }

   executions.iter()
    .map(|vals| vals.last().unwrap())
    .fold(0, |acc, v| acc + v)
}

fn seq_differences(seq: &Vec<i128>) -> Vec<i128> {
    seq.windows(2).map(|ab| ab[1] - ab[0]).collect()
}

pub fn part2(file_path: String) -> u64 {

    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);

    todo!();
}

pub fn lcm(nums: &[u128]) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = format!("src/{}/sample_1", DAY).to_string();
        let expected = 114;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: i128 = 2;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    // #[test]
    // pub fn test_sample_2(){
    //     let test_file = format!("src/{}/sample_2", DAY).to_string();
    //     let expected = 6;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }

    // #[test]
    // pub fn test_input_2(){
    //     let test_file = format!("src/{}/input", DAY).to_string();
    //     let expected: u64 = 2;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }
}