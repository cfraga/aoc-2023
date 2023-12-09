use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, num::Wrapping, sync::Arc, borrow::{BorrowMut, Borrow}};
use array_tool::vec::Intersect;

use log::debug;

const DAY: &str="day8";

#[derive(Debug, Clone)]
pub struct Node {
    val: String,
    l: String,
    r: String,
}

pub fn part1(file_path: String) -> u64 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut directions = "".to_string();
    let mut nodes = HashMap::new();

    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        if n == 0 {
            directions = l.clone();
        } else {
            let node = parse_node(l);
            nodes.insert(node.val.clone(), node);
        }
    }

    let mut i = 0;
    let mut curr_node = nodes.get("AAA").unwrap();
    while true {
        let curr_char = &directions.as_bytes()[i % directions.len()];
        i +=1;
        let n_node = match *curr_char as char {
            'L' => &curr_node.l,
            'R' => &curr_node.r,
            _ => panic!("wut??"),
        };
        
        println!("char:{}, curr:{:?}, next:{}", curr_char, curr_node, n_node);
        if n_node == "ZZZ" {
            break;
        }

        curr_node = nodes.get(n_node).unwrap();
    }

    i as u64
}

// AAA = (BBB, CCC)
fn parse_node<'a>(line: String) -> Node {
    let parts = line.split(" = ").map(|s| s.to_string()).collect::<Vec<String>>();
    let dirs = parts[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
    
    Node { val: parts[0].clone(), l: dirs[0][1..dirs[1].len()].to_string(), r: dirs[1][0..dirs[1].len()-1].to_string()}
}


pub fn part2(file_path: String) -> u64 {

    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut directions = "".to_string();
    let mut nodes = HashMap::new();

    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        if n == 0 {
            directions = l.clone();
        } else {
            let node = parse_node(l);
            nodes.insert(node.val.clone(), node);
        }
    }

    let mut i = 0;
    let mut curr_nodes = nodes.values().filter(|n| n.val.ends_with("A")).collect::<Vec<&Node>>();
    let mut first_iter = vec![0; curr_nodes.len()];

    while true {
        let curr_char = &directions.as_bytes()[i % directions.len()];
        i +=1;
        let n_nodes = curr_nodes.iter()
            .enumerate()
            // .filter(|n| !n.1.val.ends_with("Z"))
            .map( |curr_node| {
                if curr_node.1.val.ends_with("Z") {
                    curr_node.1
                } else {
                    let next_n = match *curr_char as char {
                        'L' => nodes.get(&curr_node.1.l).unwrap(),
                        'R' => nodes.get(&curr_node.1.r).unwrap(),
                        _ => panic!("wut??"),
                    };
                    if next_n.val.ends_with("Z") {
                        first_iter[curr_node.0] = i as u128;
                    }
                    next_n
                }
                
            }).collect::<Vec<&Node>>();
        
        println!("char:{}\tpath:{}\n\tcurr:{:?}\n\tnext:{:?}", curr_char, i, curr_nodes, n_nodes);
        if n_nodes.iter().all(|n| n.val.ends_with("Z")) {
            break;
        }

        
        curr_nodes = n_nodes;
    }

    println!("iters: {:?}", first_iter);
    lcm(&first_iter) as u64
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
        let expected = 2;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: u64 = 22411;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = format!("src/{}/sample_2", DAY).to_string();
        let expected = 6;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: u64 = 2;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}