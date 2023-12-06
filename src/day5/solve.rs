use std::{fs::File, io::{BufReader, BufRead}};
use array_tool::vec::Intersect;
use log::debug;
use regex::{Regex, Captures};

const DAY: &str="day5";

pub struct MappingEntry {
    from: u64,
    to: u64,
    length: u64,
}

impl MappingEntry {
    fn dest(&self, seed: &u64) -> Option<u64> {
        if seed >= &self.from && seed < &(&self.length + &self.from) {
            Some(self.to + seed - self.from)
        } else {
            None
        }
    }
}   

pub enum MapType {
    SeedSoil,
    SoilFertilizer,
    FertilizerWater,
    WaterLight,
    LightTemp,
    TempHumid,
    HumidLocation,
}

pub fn part1(file_path: String) -> u64 { 
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut seeds = Vec::new();
    let mut map_type = MapType::SeedSoil;
    let mut m_seed_soil:Vec<MappingEntry> = Vec::new();
    let mut m_soil_fert:Vec<MappingEntry> = Vec::new();
    let mut m_fert_water:Vec<MappingEntry> = Vec::new();
    let mut m_water_light:Vec<MappingEntry> = Vec::new();
    let mut m_light_temp:Vec<MappingEntry> = Vec::new();
    let mut m_temp_humid:Vec<MappingEntry> = Vec::new();
    let mut m_humid_location:Vec<MappingEntry> = Vec::new();
    
    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        if n == 0 {
            seeds = parse_seeds(l);
            continue;
        }
        if l.is_empty() { continue; };

        if l.contains(" map:") {
            debug!("parsing map {:?}",l);
            map_type = match l.split(" map:").take(1).collect::<Vec<&str>>()[0] {
                "seed-to-soil" => MapType::SeedSoil,
                "soil-to-fertilizer" => MapType::SoilFertilizer,
                "fertilizer-to-water" => MapType::FertilizerWater,
                "water-to-light" => MapType::WaterLight,
                "light-to-temperature" => MapType::LightTemp,
                "temperature-to-humidity" => MapType::TempHumid,
                "humidity-to-location" => MapType::HumidLocation,
                _ => panic!("wut? no map?")
            };
            continue;
        }

        match map_type {
            MapType::SeedSoil => m_seed_soil.push(parse_map_line(l)),
            MapType::SoilFertilizer => m_soil_fert.push(parse_map_line(l)),
            MapType::FertilizerWater => m_fert_water.push(parse_map_line(l)),
            MapType::WaterLight => m_water_light.push(parse_map_line(l)),
            MapType::LightTemp => m_light_temp.push(parse_map_line(l)),
            MapType::TempHumid => m_temp_humid.push(parse_map_line(l)),
            MapType::HumidLocation => m_humid_location.push(parse_map_line(l)),
        }
    }

    seeds.iter()
        .map (|seed| 
            m_seed_soil.iter()
            .find(|&m| m.dest(seed).is_some())
            .and_then(|m| m.dest(seed))
            .unwrap_or(*seed))
        .map (|soil| 
            m_soil_fert.iter()
            .find(|&m| m.dest(&soil).is_some())
            .and_then(|m| m.dest(&soil))
            .unwrap_or(soil))
        .map (|fert| 
            m_fert_water.iter()
            .find(|&m| m.dest(&fert).is_some())
            .and_then(|m| m.dest(&fert))
            .unwrap_or(fert))
        .map (|water| 
            m_water_light.iter()
            .find(|&m| m.dest(&water).is_some())
            .and_then(|m| m.dest(&water))
            .unwrap_or(water))
        .map (|light| 
            m_light_temp.iter()
            .find(|&m| m.dest(&light).is_some())
            .and_then(|m| m.dest(&light))
            .unwrap_or(light))
        .map (|temp| 
            m_temp_humid.iter()
            .find(|&m| m.dest(&temp).is_some())
            .and_then(|m| m.dest(&temp))
            .unwrap_or(temp))
        .map (|humid| 
            m_humid_location.iter()
            .find(|&m| m.dest(&humid).is_some())
            .and_then(|m| m.dest(&humid))
            .unwrap_or(humid))
        .inspect(|location| debug!("final location: {}", location))
        .min().unwrap()
}

// sample:
// seeds: 79 14 55 13
fn parse_seeds(line: String) -> Vec<u64> {
    line
        .split("seeds: ").skip(1)
        .flat_map(|numbers| numbers.split_whitespace().map(|n| n.parse::<u64>().unwrap()))
        .collect()
}

// sample:
// seeds: 79 14 55 13
fn parse_seeds_2(line: String) -> Vec<u64> {
    line
        .split("seeds: ").skip(1)
        .flat_map(|numbers| numbers.split_whitespace().map(|n| n.parse::<u64>().unwrap()))
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|ns| ns[0]..ns[0] + ns[1]-1)
        .collect()
}


fn parse_map_line(line: String) -> MappingEntry {
    let numbers = line.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    debug!("line: numbers: {:?}", numbers);
    MappingEntry { from: numbers[1], to: numbers[0], length: numbers[2] }
}

pub fn part2(file_path: String) -> u64 {
    let f = File::open(file_path).expect("couldnt open file");
    let reader = BufReader::new(f);
    let mut seeds = Vec::new();
    let mut map_type = MapType::SeedSoil;
    let mut m_seed_soil:Vec<MappingEntry> = Vec::new();
    let mut m_soil_fert:Vec<MappingEntry> = Vec::new();
    let mut m_fert_water:Vec<MappingEntry> = Vec::new();
    let mut m_water_light:Vec<MappingEntry> = Vec::new();
    let mut m_light_temp:Vec<MappingEntry> = Vec::new();
    let mut m_temp_humid:Vec<MappingEntry> = Vec::new();
    let mut m_humid_location:Vec<MappingEntry> = Vec::new();
    
    for (n, l) in reader.lines().flat_map( |maybe_l| maybe_l.ok()).enumerate() {
        if n == 0 {
            seeds = parse_seeds_2(l);
            continue;
        }
        if l.is_empty() { continue; };

        if l.contains(" map:") {
            debug!("parsing map {:?}",l);
            map_type = match l.split(" map:").take(1).collect::<Vec<&str>>()[0] {
                "seed-to-soil" => MapType::SeedSoil,
                "soil-to-fertilizer" => MapType::SoilFertilizer,
                "fertilizer-to-water" => MapType::FertilizerWater,
                "water-to-light" => MapType::WaterLight,
                "light-to-temperature" => MapType::LightTemp,
                "temperature-to-humidity" => MapType::TempHumid,
                "humidity-to-location" => MapType::HumidLocation,
                _ => panic!("wut? no map?")
            };
            continue;
        }

        match map_type {
            MapType::SeedSoil => m_seed_soil.push(parse_map_line(l)),
            MapType::SoilFertilizer => m_soil_fert.push(parse_map_line(l)),
            MapType::FertilizerWater => m_fert_water.push(parse_map_line(l)),
            MapType::WaterLight => m_water_light.push(parse_map_line(l)),
            MapType::LightTemp => m_light_temp.push(parse_map_line(l)),
            MapType::TempHumid => m_temp_humid.push(parse_map_line(l)),
            MapType::HumidLocation => m_humid_location.push(parse_map_line(l)),
        }
    }

    seeds.iter()
        .map (|seed| 
            m_seed_soil.iter()
            .find(|&m| m.dest(seed).is_some())
            .and_then(|m| m.dest(seed))
            .unwrap_or(*seed))
        .map (|soil| 
            m_soil_fert.iter()
            .find(|&m| m.dest(&soil).is_some())
            .and_then(|m| m.dest(&soil))
            .unwrap_or(soil))
        .map (|fert| 
            m_fert_water.iter()
            .find(|&m| m.dest(&fert).is_some())
            .and_then(|m| m.dest(&fert))
            .unwrap_or(fert))
        .map (|water| 
            m_water_light.iter()
            .find(|&m| m.dest(&water).is_some())
            .and_then(|m| m.dest(&water))
            .unwrap_or(water))
        .map (|light| 
            m_light_temp.iter()
            .find(|&m| m.dest(&light).is_some())
            .and_then(|m| m.dest(&light))
            .unwrap_or(light))
        .map (|temp| 
            m_temp_humid.iter()
            .find(|&m| m.dest(&temp).is_some())
            .and_then(|m| m.dest(&temp))
            .unwrap_or(temp))
        .map (|humid| 
            m_humid_location.iter()
            .find(|&m| m.dest(&humid).is_some())
            .and_then(|m| m.dest(&humid))
            .unwrap_or(humid))
        .inspect(|location| debug!("final location: {}", location))
        .min().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_1(){
        let test_file = format!("src/{}/sample_1", DAY).to_string();
        let expected = 35;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_1(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: u64 = 662197086;

        let result = part1(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_sample_2(){
        let test_file = format!("src/{}/sample_1", DAY).to_string();
        let expected = 46;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_input_2(){
        let test_file = format!("src/{}/input", DAY).to_string();
        let expected: u64 = 52510809;

        let result = part2(test_file);

        assert_eq!(result, expected);
    }
}