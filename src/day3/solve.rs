use std::{fs::File, io::{BufReader, BufRead}};
use array_tool::vec::Intersect;
use log::debug;
use regex::{Regex, Captures};

#[derive(Clone, Debug)]
pub struct PartNumber {
    val: String,
    start: [u8; 2],
}

impl PartNumber {
    fn coords(&self) -> Vec<[u8; 2]> {
        let mut res = Vec::new();

        for i in 0..self.val.len() {
            res.push([self.start[0] + (i as u8), self.start[1]]);
        }

        res
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Part {
    val: char,
    pos: [u8; 2],
}

pub fn part1(file_path: String) -> u32 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut numbers = Vec::new();
    let mut parts = Vec::new();
    
    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        let (n, p) = parse_line(l, n);
        numbers.extend(n);
        parts.extend(p);
    }
   
    calculate_value(numbers, parts)
}

fn parse_line(line: String, n: usize) -> (Vec<PartNumber>, Vec<Part>) {
    let mut i = 0;
    let mut numbers: Vec<PartNumber> = Vec::new();
    let mut parts = Vec::new();
    debug!("parsing line {}", n);
    let mut in_digits = false;

    for c in line.chars() {
        i+=1;
        match c {
            '.' => in_digits = false,
            d@ '0'..='9' => {
                debug!("found digit {}: {:?}", d, in_digits);
                if in_digits {
                    let mut current_el = numbers.pop().unwrap();
                    current_el.val.push(d);
                    debug!("{:?}", current_el);
                    numbers.push(current_el);
                } else {
                    numbers.push(PartNumber { val: d.to_string(), start: [i, n.try_into().unwrap()] });
                };
                in_digits = true
            },
            p @ _ => {
               in_digits = false;
               parts.push(Part { val: p.clone(), pos: [i, n.try_into().unwrap()] })
            },
        }
    }

    (numbers, parts)
}

pub fn part2(file_path: String) -> u32 {
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut numbers = Vec::new();
    let mut parts = Vec::new();
    
    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        let (n, p) = parse_line(l, n);
        numbers.extend(n);
        parts.extend(p);
    }
   
    calculate_gear_ratios(numbers, parts)
}


fn calculate_gear_ratios(numbers: Vec<PartNumber>, parts: Vec<Part>) -> u32 {
    parts.iter()
        .filter(|p| p.val == '*')
        .map(|p| gear_ratio(&numbers, p).unwrap_or(0))
        .sum()
}

fn gear_ratio(numbers: &Vec<PartNumber>, p: &Part) -> Option<u32> {
    let coords = vec![
        [p.pos[0]-1, p.pos[1]-1],
        [p.pos[0]-1, p.pos[1]],
        [p.pos[0]-1, p.pos[1]+1],
        [p.pos[0], p.pos[1]-1],
        [p.pos[0], p.pos[1]],
        [p.pos[0], p.pos[1]+1],
        [p.pos[0]+1, p.pos[1]-1],
        [p.pos[0]+1, p.pos[1]],
        [p.pos[0]+1, p.pos[1]+1],
    ];
    println!("gear {:?}", p);

    let ratios: Vec<u32> = numbers
        .iter()
        .filter (|n| {
             !n.coords().intersect(coords.clone()).is_empty()
        })
        .inspect(|n| print!(" {:?}", n))
        .map(|n| n.val.parse().unwrap())
        .collect();

    match ratios.len() == 2 {
        false => None,
        true => Some(ratios[0] * ratios[1]),
    }
}

fn calculate_value(numbers: Vec<PartNumber>, parts: Vec<Part>) -> u32 {
   parts
    .iter()
    .map(|p| {
        part_numbers(&numbers, p).iter().sum::<u32>()
    })
    .sum()
}

fn part_numbers(numbers: &Vec<PartNumber>, p: &Part) -> Vec<u32> {
    let coords = vec![
        [p.pos[0]-1, p.pos[1]-1],
        [p.pos[0]-1, p.pos[1]],
        [p.pos[0]-1, p.pos[1]+1],
        [p.pos[0], p.pos[1]-1],
        [p.pos[0], p.pos[1]],
        [p.pos[0], p.pos[1]+1],
        [p.pos[0]+1, p.pos[1]-1],
        [p.pos[0]+1, p.pos[1]],
        [p.pos[0]+1, p.pos[1]+1],
    ];
    numbers.iter().filter (|n| {
       !n.coords().intersect(coords.clone()).is_empty()
    })
    .inspect(|n| print!(" {:?}", n))
    .map(|n| n.val.parse().unwrap())
    .collect()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = "src/day3/sample_1".to_string();
        let expected = 4361;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = "src/day3/input".to_string();
        let expected: u32 = 520019;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = "src/day3/sample_1".to_string();
        let expected = 467835;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = "src/day3/input".to_string();
        let expected: u32 = 75519888;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}