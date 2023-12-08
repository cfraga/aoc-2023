use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, num::Wrapping};
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
    todo!();
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
        let expected: u64 = 2;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    // #[test]
    // pub fn test_sample_2(){
    //     let test_file = format!("src/{}/sample_1", DAY).to_string();
    //     let expected = 5905;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }

    // #[test]
    // pub fn test_input_2(){
    //     let test_file = format!("src/{}/input", DAY).to_string();
    //     let expected: u64 = 253473930;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }
}