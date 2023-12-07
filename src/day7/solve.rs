use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, num::Wrapping};
use array_tool::vec::Intersect;
use log::debug;

const DAY: &str="day7";

#[derive(Debug)]
pub struct Hand {
    hand: String,
    strength: u32,
    kind: HandType,
    bid: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

pub fn part1(file_path: String) -> u64 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut hands = reader
        .lines()
        .flat_map( |maybe_l| maybe_l.ok()).map(|l| parse_line(l))
        .inspect(|h| println!("hand: {:?}", h))
        .collect::<Vec<Hand>>();

    hands.sort_by(|a,b| b.strength.cmp(&a.strength));

    hands.iter()
        .enumerate()
        .map( |(i, h)| h.bid * (hands.len() as u64 - i as u64))
        .sum()
}

fn parse_line(line: String) -> Hand {
    let parts:Vec<&str> = line.split_whitespace().collect();
    let mut repetitions = HashMap::new();
    
    let hand = parts[0].chars()
    .map ( |c| {
        let new_c = match c {
            'A' => 'e',
            'K' => 'd',
            'Q' => 'c',
            'J' => 'b',
            'T' => 'a',
            val @ _ => val,
        };
        *repetitions.entry(new_c).or_insert(0) += 1;
        
        new_c
    }).collect::<String>();

    let hand_type = hand_type(&repetitions);
    let bid = parts[1].parse::<u64>().unwrap();
            
    Hand { hand: hand.clone(), kind: hand_type, bid: bid, strength: calc_strength(hand_type, hand)}
}

fn calc_strength(hand_type: HandType, hand: String) -> u32 {
    let mut hand_val = hand_type as u32 * (16u32.pow(6));
    println!("hand_type {:?}, hand {:?}", hand_val, hand);
    let hand_hex_bytes = hand.chars().map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap()).collect::<Vec<u8>>();
    let mut remaining_digits = hand.len() as u32;

    for hex in hand_hex_bytes.iter() {
        hand_val += *hex as u32 * (16u32.pow(remaining_digits));
        println!("handval {:?}", hand_val);
        remaining_digits -= 1;
    }

    hand_val
}

fn hand_type(reps: &HashMap<char,u32>) -> HandType {
    let mut count_vec = reps.iter().collect::<Vec<(&char, &u32)>>();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("hand count {:?}", count_vec);

    if count_vec.len() > 3 {
        if *count_vec[0].1 == 2 {
            return HandType::OnePair;
        } else { 
         return HandType::HighCard;
        }
    }
    if count_vec.len() == 3 {
        if *count_vec[0].1 == 3 {
            return HandType::ThreeKind;
        } else {
            return HandType::TwoPair;
        }
    }

    if *count_vec[0].1 == 4 {
        return HandType::FourKind;
    }
    
    if count_vec.len() == 2 {
        return HandType::FullHouse;
    }

    if *count_vec[0].1 == 5 {
        return HandType::FiveKind;
    } 
    

    panic!("wat?! dunno what hand type this is");
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
        let expected = 6440;

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

    // #[test]
    // pub fn test_sample_2(){
    //     let test_file = format!("src/{}/sample_2", DAY).to_string();
    //     let expected = 71503;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }

    // #[test]
    // pub fn test_input_2(){
    //     let test_file = format!("src/{}/input2", DAY).to_string();
    //     let expected: u128 = 52510809;

    //     let result = part2(test_file);

    //     assert_eq!(result, expected);
    // }
}