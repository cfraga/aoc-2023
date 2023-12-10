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

fn calc_sequence_first(seq: Vec<i128>) -> i128 {
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
    .inspect(|v| debug!("vals {:?}", v)) 
    .map(|vals| vals.first().unwrap())
    .rev()
    .fold(0, |acc, v| { debug!("\t{}", acc); v - acc } )
}

fn seq_differences(seq: &Vec<i128>) -> Vec<i128> {
    seq.windows(2).map(|ab| ab[1] - ab[0]).collect()
}

pub fn part2(file_path: String) -> i128 {

    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);

    reader.lines()
    .flat_map( |maybe_l| maybe_l.ok())
    .map(|l| l.split_whitespace().map(|n| n.parse::<i128>().unwrap()).collect())
    .map(|sequence| calc_sequence_first(sequence))
    .inspect(|val| debug!("total {}", val))
    .sum()
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
        let expected: i128 = 1953784198;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = format!("src/{}/sample_1", DAY).to_string();
        let expected: i128 = 2;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: i128 = 957;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}