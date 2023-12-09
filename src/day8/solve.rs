use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, num::Wrapping, sync::Arc, borrow::{BorrowMut, Borrow}};
use array_tool::vec::Intersect;
use dashmap::DashMap;
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
        let mut n_nodes_copy = vec![];

        let n_nodes = curr_nodes.iter().filter(|n| !n.val.ends_with("Z")).map( |curr_node| {
            match *curr_char as char {
                'L' => nodes.get(&curr_node.l).unwrap(),
                'R' => nodes.get(&curr_node.r).unwrap(),
                _ => panic!("wut??"),
            }
        }).collect::<Vec<&Node>>();
        
        println!("char:{}\n\tcurr:{:?}\n\tnext:{:?}", curr_char, curr_nodes, n_nodes);
        if n_nodes_copy.iter().all(|n| n.val.ends_with("Z")) {
            break;
        }

        curr_nodes = n_nodes;
    }

    i as u64
}

fn traverse_to_z(directions: &String, nodes: DashMap<String, Node>, curr_i: u32, node: &Node) -> (String, u32) {
    let mut i = curr_i;
    let mut curr_node = node;
    loop {
        let curr_char = &directions.as_bytes()[i as usize % directions.len()];
        i +=1;

        let n_node = match *curr_char as char {
                'L' => nodes.get(&curr_node.l).unwrap().borrow(),
                'R' => nodes.get(&curr_node.r).unwrap().borrow(),
                _ => panic!("wut??"),
            };
        
        println!("char:{}\n\tcurr:{:?}\n\tnext:{:?}", curr_char, curr_node, n_node);
        if n_node.val.ends_with("Z") {
            break;
        }

        curr_node = n_node;
    }

    (curr_node.val, i)

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