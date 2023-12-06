use rayon::prelude::*;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

#[derive(PartialEq, Eq, Hash)]
enum Mode {
    Unknown,
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumitidyToLocation,
}

fn part1_solve(input: &str) -> u64 {
    let lines = input.trim().split('\n');

    let mappings: HashMap<Mode, HashMap<u64, u64>> = HashMap::from([
        (Mode::SeedToSoil, HashMap::new()),
        (Mode::SoilToFertilizer, HashMap::new()),
        (Mode::FertilizerToWater, HashMap::new()),
        (Mode::WaterToLight, HashMap::new()),
        (Mode::LightToTemperature, HashMap::new()),
        (Mode::TemperatureToHumidity, HashMap::new()),
        (Mode::HumitidyToLocation, HashMap::new()),
    ]);

    let label_to_mode = HashMap::from([
        ("seed-to-soil", Mode::SeedToSoil),
        ("soil-to-fertilizer", Mode::SoilToFertilizer),
        ("fertilizer-to-water", Mode::FertilizerToWater),
        ("water-to-light", Mode::WaterToLight),
        ("light-to-temperature", Mode::LightToTemperature),
        ("temperature-to-humidity", Mode::TemperatureToHumidity),
        ("humidity-to-location", Mode::HumitidyToLocation),
    ]);

    let mut seed_numbers: Vec<u64> = Vec::new();

    let mut mode = &Mode::Unknown;
    let mut all_mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut current_mapping: Vec<(u64, u64, u64)> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds") {
            mode = &Mode::Seeds;
            println!("SEEDS: {}", line);
            seed_numbers = line
                .replace("seeds: ", "")
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            continue;
        }

        if line.contains("map") {
            // Append the previous mapping to the list of mappings
            if mode != &Mode::Seeds {
                all_mappings.push(current_mapping);
            }

            // Start the new mapping
            let first_word = line.split(' ').next().unwrap();
            mode = label_to_mode.get(first_word).unwrap();
            current_mapping = Vec::new();
            continue;
        }

        // Verify we're in a mode that supports mapping
        assert!(mappings.get(mode).is_some());

        let numbers: Vec<u64> = line.split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        let destination_range_start = *numbers.get(0).unwrap();
        let source_range_start = *numbers.get(1).unwrap();
        let range_length = *numbers.get(2).unwrap();

        current_mapping.push((destination_range_start, source_range_start, range_length));
    }
    all_mappings.push(current_mapping);

    seed_numbers
        .par_iter()
        .map(|seed_number| {
            let mut current_value = *seed_number;
            for single_mapping in &all_mappings {
                for (destination_range_start, source_range_start, range_length) in single_mapping {
                    let source_range_end = source_range_start + range_length - 1;
                    if current_value >= *source_range_start && current_value <= source_range_end {
                        let distance_into_range = current_value - source_range_start;
                        current_value = destination_range_start + distance_into_range;
                        break;
                    }
                }
            }
            current_value
        })
        .min()
        .unwrap()
}

fn part2_solve(input: &str) -> u64 {
    let lines = input.trim().split('\n');

    let mappings: HashMap<Mode, HashMap<u64, u64>> = HashMap::from([
        (Mode::SeedToSoil, HashMap::new()),
        (Mode::SoilToFertilizer, HashMap::new()),
        (Mode::FertilizerToWater, HashMap::new()),
        (Mode::WaterToLight, HashMap::new()),
        (Mode::LightToTemperature, HashMap::new()),
        (Mode::TemperatureToHumidity, HashMap::new()),
        (Mode::HumitidyToLocation, HashMap::new()),
    ]);

    let label_to_mode = HashMap::from([
        ("seed-to-soil", Mode::SeedToSoil),
        ("soil-to-fertilizer", Mode::SoilToFertilizer),
        ("fertilizer-to-water", Mode::FertilizerToWater),
        ("water-to-light", Mode::WaterToLight),
        ("light-to-temperature", Mode::LightToTemperature),
        ("temperature-to-humidity", Mode::TemperatureToHumidity),
        ("humidity-to-location", Mode::HumitidyToLocation),
    ]);

    let mut seed_number_ranges: Vec<(u64, u64)> = Vec::new();

    let mut mode = &Mode::Unknown;
    let mut all_mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut current_mapping: Vec<(u64, u64, u64)> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds") {
            mode = &Mode::Seeds;
            println!("SEEDS: {}", line);
            let raw_seed_numbers: Vec<u64> = line
                .replace("seeds: ", "")
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            for seed_pair_index in (0..raw_seed_numbers.len()).step_by(2) {
                let seed_number_range = (
                    *raw_seed_numbers.get(seed_pair_index).unwrap(),
                    *raw_seed_numbers.get(seed_pair_index + 1).unwrap(),
                );
                seed_number_ranges.push(seed_number_range);
            }
            continue;
        }

        if line.contains("map") {
            // Append the previous mapping to the list of mappings
            if mode != &Mode::Seeds {
                all_mappings.push(current_mapping);
            }

            // Start the new mapping
            let first_word = line.split(' ').next().unwrap();
            mode = label_to_mode.get(first_word).unwrap();
            current_mapping = Vec::new();
            continue;
        }

        // Verify we're in a mode that supports mapping
        assert!(mappings.get(mode).is_some());

        let numbers: Vec<u64> = line.split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        let destination_range_start = *numbers.get(0).unwrap();
        let source_range_start = *numbers.get(1).unwrap();
        let range_length = *numbers.get(2).unwrap();

        current_mapping.push((destination_range_start, source_range_start, range_length));
    }
    all_mappings.push(current_mapping);

    seed_number_ranges
        .par_iter()
        .map(|(seed_number_start, seed_number_range_length)| {
            println!(
                "SEED RANGE, {} .. {}",
                seed_number_start,
                seed_number_start + seed_number_range_length
            );
            (*seed_number_start..seed_number_start + seed_number_range_length)
                .into_par_iter()
                .map(|seed_number| {
                    let mut current_value = seed_number;
                    for single_mapping in &all_mappings {
                        for (destination_range_start, source_range_start, range_length) in
                            single_mapping
                        {
                            let source_range_end = source_range_start + range_length - 1;
                            if current_value >= *source_range_start
                                && current_value <= source_range_end
                            {
                                let distance_into_range = current_value - source_range_start;
                                current_value = destination_range_start + distance_into_range;
                                break;
                            }
                        }
                    }
                    current_value
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(35, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(46, part2_solve(&input));
    }
}
