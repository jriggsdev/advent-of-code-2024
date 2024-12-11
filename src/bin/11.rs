advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = vec!();
    let values = input
        .lines()
        .next()?
        .split(' ')
        .filter(|word| !word.is_empty());

    for value in values {
        stones.push(value.parse().unwrap());
    }

    let mut stones_swap: Vec<u64> = vec!();
    for i in 0..25 {
        if i % 2 == 0 {
            stones_swap = vec!();
            for stone in stones.iter() {
                if stone == &0 {
                    stones_swap.push(1)
                } else {
                    let tens: u32 = (*stone as f64).log10().floor() as u32 + 1;
                    if tens % 2 == 0 {
                        let half = 10_u32.pow(tens/2);
                        let first = stone / half as u64;
                        let second = stone % half as u64;
                        stones_swap.push(first);
                        stones_swap.push(second);
                    } else {
                        stones_swap.push(stone * 2024)
                    }
                }
            }
        } else {
            stones = vec!();
            for stone in stones_swap.iter() {
                if stone == &0 {
                    stones.push(1)
                } else {
                    let tens: u32 = (*stone as f64).log10().floor() as u32 + 1;
                    if tens % 2 == 0 {
                        let half = 10_u32.pow(tens/2);
                        let first = stone / half as u64;
                        let second = stone % half as u64;
                        stones.push(first);
                        stones.push(second);
                    } else {
                        stones.push(stone * 2024)
                    }
                }
            }
        }
    }

    Some(stones_swap.len() as u64)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
