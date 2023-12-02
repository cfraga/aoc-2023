use std::{fs::File, io::{BufReader, BufRead}, str::FromStr};

use log::debug;
use regex::{Regex, Captures};

pub struct GameInfo {
    id: u32,
    extractions: Vec<RGB>,

}

impl GameInfo {
    fn power(&self) -> u32{
        self.extractions.iter().map(|e| e.r).max().unwrap() as u32 *
        self.extractions.iter().map(|e| e.g).max().unwrap() as u32 *
        self.extractions.iter().map(|e| e.b).max().unwrap() as u32
    }
}

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    // expecting a game input slice. In other words, one of the bits within brackets in the comment below
    // Game 1: [3 blue, 4 red]; [1 red, 2 green, 6 blue]; [2 green
    fn from_input_slice(s: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        debug!("{}", s);
        for c in s.split(", ") {
            let ele = c.split(" ").collect::<Vec<&str>>();
            match ele[1] {
                "red" => r = ele[0].parse::<u8>().unwrap(),
                "green" => g = ele[0].parse::<u8>().unwrap(),
                "blue" => b = ele[0].parse::<u8>().unwrap(),
                other => panic!("wat? {}", other),
            }
        };

        RGB { r, g, b }
    }
}

pub fn part1(file_path: String) -> u32 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let max_bag_trinkets = RGB { r: 12, g: 13, b: 14};
    
    reader.lines()
        .flat_map( |maybe_l| maybe_l.ok())
        .map(|l| parse_game(l))
        .filter(|g| elligible_game(g, &max_bag_trinkets))
        .map(|g| g.id)
        .sum()

}

pub fn part2(file_path: String) -> u32 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    
    reader.lines()
        .flat_map( |maybe_l| maybe_l.ok())
        .map(|l| parse_game(l))
        .map(|g| g.power())
        .sum()

}


fn parse_game(l: String) -> GameInfo {
    let elements: Vec<String> = l
        .split(": ")
        .map(|s| s
            .split("; ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
        .flatten()
        .collect();

    let game_id = elements[0].split(" ").collect::<Vec<&str>>().last().unwrap().to_string().parse::<u32>().unwrap();
    let extractions = elements[1..].into_iter().map(|s| RGB::from_input_slice(s)).collect();

    GameInfo { id: game_id, extractions: extractions }
}

fn elligible_game(game: &GameInfo, maxvals: &RGB) -> bool {
    game.extractions.iter().all(|e| e.r <= maxvals.r && e.g <= maxvals.g && e.b <= maxvals.b )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = "src/day2/sample_1".to_string();
        let expected = 8;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = "src/day2/input".to_string();
        let expected: u32 = 2164;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }
    #[test]
    pub fn test_sample_2(){
        let test_file = "src/day2/sample_2".to_string();
        let expected = 2286;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = "src/day2/input".to_string();
        let expected: u32 = 69929;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}