advent_of_code::solution!(2);

fn is_good_report<I: Iterator<Item = i32>>(levels: &mut I) -> bool {
    let mut current_level = levels.next().unwrap();
    let mut ascending:Option<bool> = None;
    let mut is_good = true;
    while let Some(next_level) = levels.next() {

        if ascending == None {
            ascending = Some(next_level > current_level);
        }

        if ascending == Some(true) && next_level <= current_level {
            is_good = false;
            break;
        }

        if ascending == Some(false) && next_level >= current_level {
            is_good = false;
            break;
        }

        let diff = next_level - current_level;
        if diff > 3 || diff < -3 {
            is_good = false;
            break;
        }

        current_level = next_level;
    }

    is_good
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut good_reports = 0;

    for report in input.lines().filter(|line| !line.is_empty()) {
        let mut levels = report.split(" ")
            .map(|item| item.parse::<i32>().unwrap());

        if is_good_report(&mut levels) {
            good_reports = good_reports + 1
        }
    }

    Some(good_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut good_reports = 0;
    for report in input.lines().filter(|line| !line.is_empty()) {
        let levels: Vec<i32> = report.split(" ")
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        let mut index = 0;
        while index < levels.len() {
            let mut dampened_levels = levels
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != index)
                .map(|(_, &val)| val);

            if is_good_report(&mut dampened_levels) {
                good_reports = good_reports + 1;
                break;
            }

            index = index + 1
        }
    }

    Some(good_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
