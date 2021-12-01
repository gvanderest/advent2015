fn do_a_hash(prefix: &str, suffix: &usize) -> String {
    let full_input = format!("{}{}", prefix, suffix);
    let hashed = md5::compute(&full_input);

    return format!("{:?}", hashed);
}

fn part1(input: &str) -> usize {
    let mut suffix = 0;

    loop {
        suffix += 1;
        let hash = do_a_hash(input, &suffix);
        if hash.starts_with("00000") {
            return suffix;
        }
    }
}

fn part2(input: &str) -> usize {
    let mut suffix = 0;

    loop {
        suffix += 1;
        let hash = do_a_hash(input, &suffix);
        if hash.starts_with("000000") {
            return suffix;
        }
    }
}

fn main() {
    println!("Part one: {}", part1("yzbqklnj"));
    println!("Part two: {}", part2("yzbqklnj"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
