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
        .inspect(|n| debug!("score: {}", n))
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

pub fn part2(file_path: String) -> u32 {
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut scratchcards: Vec<[Vec<u8>; 2]> = Vec::new();
    let mut processed_cards: Vec<u32> = Vec::new();
    
    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        let (l, r) = parse_line(l);
        scratchcards.push([l, r]);
        processed_cards.push(1);
    }

    for i in 0..scratchcards.len() {
        debug!("processing card {} with {}", i, processed_cards[i]);
        let num_matches = winning_numbers(&scratchcards[i][0], &scratchcards[i][1]);
        if num_matches > 0 {
            for n_i in i+1..i+1+num_matches as usize {
                if n_i < scratchcards.len() {
                    debug!("adding {} to {}", processed_cards[i],processed_cards[n_i]);
                    processed_cards[n_i] += processed_cards[i];
                }
            }
        }
    }
    debug!("processed_c {:?}", processed_cards);
    processed_cards.iter().sum()
}

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
        let expected: u32 = 22488;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = "src/day4/sample_1".to_string();
        let expected = 30;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = "src/day4/input".to_string();
        let expected: u32 = 7013204;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}