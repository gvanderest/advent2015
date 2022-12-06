use std::fs;

fn main() {
    let input = String::from(
        fs::read_to_string("input.txt")
            .expect("input.txt exists")
            .trim(),
    );
    println!("Part 1: {}", part1_process(&input));
    println!("Part 2: {}", part2_process(&input));
}

fn part1_process(input: &String) -> i32 {
    let split_calories = input.split("\n\n");
    let mut calories_per_elf = vec![];
    for calorie_batch in split_calories {
        let elf_meals = calorie_batch.split("\n");
        let mut elf_total_calories = 0;
        for raw_meal in elf_meals {
            let meal_calories = raw_meal.parse::<i32>().expect("valid meal calories number");
            elf_total_calories = elf_total_calories + meal_calories;
        }
        calories_per_elf.push(elf_total_calories);
    }

    let mut max_calories = 0;
    for entry in calories_per_elf {
        if entry > max_calories {
            max_calories = entry;
        }
    }

    max_calories
}

fn part2_process(input: &String) -> i32 {
    let split_calories = input.split("\n\n");
    let mut calories_per_elf = vec![];
    for calorie_batch in split_calories {
        let elf_meals = calorie_batch.split("\n");
        let mut elf_total_calories = 0;
        for raw_meal in elf_meals {
            let meal_calories = raw_meal.parse::<i32>().expect("valid meal calories number");
            elf_total_calories = elf_total_calories + meal_calories;
        }
        calories_per_elf.push(elf_total_calories);
    }

    calories_per_elf.sort();
    calories_per_elf.reverse();

    let mut total = 0;
    for cals in calories_per_elf[0..3].iter() {
        total = total + cals;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_works() {
        let input = fs::read_to_string("example.txt").expect("example.txt exists");
        assert_eq!(part1_process(&input), 24_000);
    }

    #[test]
    fn part2_example_works() {
        let input = fs::read_to_string("example.txt").expect("example.txt exists");
        assert_eq!(part2_process(&input), 45_000);
    }
}
