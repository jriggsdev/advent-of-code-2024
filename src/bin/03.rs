use regex::{Regex};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let muls = re.captures_iter(input)
        .map(|c| c.iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
            .collect::<Vec<u32>>());

    let mut total = 0;
    for mul in muls {
        total += mul[0] * mul[1];
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
