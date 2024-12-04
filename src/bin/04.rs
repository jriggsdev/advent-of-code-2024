advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars()
            .filter(|char| char != &'\n')
            .collect()
        ).collect();

    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] != 'X' {
                continue;
            }

            let check_up = i >= 3;
            let check_down = i < lines.len() - 3;
            let check_left = j >= 3;
            let check_right = j < lines[i].len() - 3;

            if check_up && lines[i-1][j] == 'M' && lines[i-2][j] == 'A' && lines[i-3][j] == 'S' {
                count += 1;
            }

            if check_down && lines[i+1][j] == 'M' && lines[i+2][j] == 'A' && lines[i+3][j] == 'S' {
                count += 1;
            }

            if check_left && lines[i][j-1] == 'M' && lines[i][j-2] == 'A' && lines[i][j-3] == 'S' {
                count += 1;
            }

            if check_right && lines[i][j+1] == 'M' && lines[i][j+2] == 'A' && lines[i][j+3] == 'S' {
                count += 1;
            }

            if check_up && check_left && lines[i-1][j-1] == 'M' && lines[i-2][j-2] == 'A' && lines[i-3][j-3] == 'S' {
                count += 1;
            }

            if check_up && check_right && lines[i-1][j+1] == 'M' && lines[i-2][j+2] == 'A' && lines[i-3][j+3] == 'S' {
                count += 1;
            }

            if check_down && check_left && lines[i+1][j-1] == 'M' && lines[i+2][j-2] == 'A' && lines[i+3][j-3] == 'S' {
                count += 1;
            }

            if check_down && check_right && lines[i+1][j+1] == 'M' && lines[i+2][j+2] == 'A' && lines[i+3][j+3] == 'S' {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars()
            .filter(|char| char != &'\n')
            .collect()
        ).collect();

    let mut count = 0;
    for i in 0..lines.len() - 2 {
        for j in 0..lines[i].len() - 2 {
            if lines[i][j] != 'M' && lines[i][j] != 'S' {
                continue;
            }

            let check_other_diagonal = if lines[i][j] == 'M' {
                lines[i+1][j+1] == 'A' && lines[i+2][j+2] == 'S'
            } else {
                lines[i+1][j+1] == 'A' && lines[i+2][j+2] == 'M'
            };

            if check_other_diagonal {
                if lines[i][j+2] == 'M' && lines[i+2][j] == 'S'
                    || lines[i][j+2] == 'S' && lines[i+2][j] == 'M' {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
