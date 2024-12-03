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
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let instructions = re.captures_iter(input)
        .map(|c| c.get(0).unwrap().as_str());

    let mut mul_on = true;
    let mut total = 0;
    for instruction in instructions {
        if instruction.starts_with("mul") && mul_on {
            let args: Vec<u32> = instruction
                .replace("mul(", "")
                .replace(")", "")
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            total += args[0] * args[1];
        } else if instruction == "don't()" {
            mul_on = false;
        } else  if instruction == "do()"{
            mul_on = true;
        }
    }

    Some(total)
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
