advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        // parse the parts of the line
        let parts: Vec<&str> = line.split(":").collect();
        let test_value = parts[0].parse::<u64>().unwrap();
        let mut numbers: Vec<u64> = parts[1].split(" ")
            .filter(|str| !str.is_empty())
            .map(|str| str.trim().parse::<u64>().unwrap())
            .collect();

        numbers.reverse();

        // tracks the remainder after an operation
        let mut remainder = test_value;

        // used as a stack to remember the indexes of divisions and remainders before applying said divisions
        let mut divisions: Vec<(usize, u64)> = vec!();

        // used to remember where we need to go back to and start trying operators
        let mut i = 0;

        // start trying combinations of divide and subtract till we've tried them all or found an answer
        loop {
            let mut stopped_early = false;
            for j in i..numbers.len() - 1 {
                // if division is possible push the index and remainder before division onto the stack
                // then calculate the remainder after division
                if remainder % numbers[j] == 0 {
                    divisions.push((j, remainder));
                    remainder /= numbers[j];
                } else {
                    // if division isn't possible first make sure subtraction is possible without
                    // going negative. If it is then subtract, if it isn't break and continue outer loop
                    if remainder <= numbers[j] {
                        stopped_early = true;
                        break;
                    }

                    remainder -= numbers[j];
                }
            }

            // if the remainder equals the first number then we are done
            if  !stopped_early && remainder == numbers[numbers.len() - 1] {
                result += test_value;
                break;
            }

            // if there are no divisions left we've tried every possible combination
            if divisions.len() == 0 {
                break;
            }

            loop {
                // pop off the top division, change it to a subtraction, and start at the top from the next index
                // if the new remainder would be 0 when applying the subtraction we can't use a subtraction here
                // so start the loop again and pop off another division
                if let Some(last_division) = divisions.pop() {
                    remainder = last_division.1 - numbers[last_division.0];
                    i = last_division.0 + 1;
                    if remainder != 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    Some(result)
}

enum Operator {
    Multiply,
    Add,
    Concat,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Multiply => a * b,
            Operator::Add => a + b,
            Operator::Concat => format!("{a}{b}").parse::<u64>().unwrap(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        // parse the parts of the line
        let parts: Vec<&str> = line.split(":").collect();
        let test_value = parts[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1].split(" ")
            .filter(|str| !str.is_empty())
            .map(|str| str.trim().parse::<u64>().unwrap())
            .collect();
        let num_operators = numbers.len() - 1;
        let num_operator_combinations = 3_u32.pow(num_operators as u32);

        for i in 0..num_operator_combinations {
            let mut operators: Vec<Operator> = vec!();
            let mut num = i;
            while num > 0 {
                let remainder = num % 3;
                match remainder {
                    0 => operators.push(Operator::Add),
                    1 => operators.push(Operator::Multiply),
                    2 => operators.push(Operator::Concat),
                    _ => panic!("Something unexpected happened. Cosmic rays flipped a bit maybe?")
                }
                num /= 3;
            }
            while operators.len() < num_operators {
                operators.push(Operator::Add);
            }

            let mut running_value = numbers[0];
            for j in 0..num_operators {
                running_value = operators[j].apply(running_value, numbers[j + 1]);
                if running_value > test_value {
                    break;
                }
            }

            if running_value == test_value {
                result += test_value;
                break;
            }
        }
    }

    Some(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
