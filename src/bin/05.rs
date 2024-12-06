advent_of_code::solution!(5);

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Update {
    pages: Vec<u32>
}

#[derive(Debug)]
enum UpdateParseError {
    InvalidUpdate
}

impl FromStr for Update {
    type Err = UpdateParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let pages: Vec<u32> = str.split(',')
            .map(|part| part.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|_| UpdateParseError::InvalidUpdate)?;

        Ok(Update { pages })
    }
}

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32
}

#[derive(Debug)]
enum RuleParseError {
    InvalidFormat,
    FirstPartInvalidInt,
    SecondPartInvalidInt,
}

impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = str.split('|').collect();

        if parts.len() != 2 {
            return Err(RuleParseError::InvalidFormat);
        }

        let first = parts[0].parse::<u32>()
            .map_err(|_| RuleParseError::FirstPartInvalidInt)?;

        let second = parts[1].parse::<u32>()
            .map_err(|_| RuleParseError::SecondPartInvalidInt)?;

        Ok(Rule { first, second, })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut rules: Vec<Rule> = vec!();
    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let rule_parse_result: Result<Rule, RuleParseError> = line.parse();

                match rule_parse_result {
                    Ok(rule) => rules.push(rule),
                    Err(rule_parse_error) => match rule_parse_error {
                        RuleParseError::InvalidFormat => {
                            panic!("Invalid rule format: {}", line);
                        },
                        RuleParseError::FirstPartInvalidInt => {
                            panic!("Invalid rule first part: {}", line);
                        },
                        RuleParseError::SecondPartInvalidInt => {
                            panic!("Invalid rule second part: {}", line);
                        }
                    }
                }
            },
            None => {
                break;
            }
        };
    }

    let mut updates: Vec<Update> = vec!();
    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let update_parse_result: Result<Update, UpdateParseError> = line.parse();

                match update_parse_result {
                    Ok(update) => updates.push(update),
                    Err(update_parse_error) => match update_parse_error {
                        UpdateParseError::InvalidUpdate => {
                            panic!("Invalid update: {}", line);
                        }
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules.iter() {
        if rules_map.contains_key(&rule.second) {
            rules_map.get_mut(&rule.second).unwrap().insert(rule.first);
        } else {
            rules_map.insert(rule.second, HashSet::from([rule.first]));
        }
    }

    let mut total:u32 = 0;
    for update in updates {
        let mut bad_pages: HashSet<u32> = HashSet::new();
        let mut good_update = true;
        for page in update.pages.iter() {
            if bad_pages.contains(&page) {
                good_update = false;
                break;
            }

            if rules_map.contains_key(&page) {
                for bad_page in rules_map.get(&page).unwrap().iter() {
                    bad_pages.insert(bad_page.clone());
                }
            }
        }

        if good_update {
            let middle_value = update.pages[update.pages.len() / 2];
            total += middle_value;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut rules: Vec<Rule> = vec!();
    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let rule_parse_result: Result<Rule, RuleParseError> = line.parse();

                match rule_parse_result {
                    Ok(rule) => rules.push(rule),
                    Err(rule_parse_error) => match rule_parse_error {
                        RuleParseError::InvalidFormat => {
                            panic!("Invalid rule format: {}", line);
                        },
                        RuleParseError::FirstPartInvalidInt => {
                            panic!("Invalid rule first part: {}", line);
                        },
                        RuleParseError::SecondPartInvalidInt => {
                            panic!("Invalid rule second part: {}", line);
                        }
                    }
                }
            },
            None => {
                break;
            }
        };
    }

    let mut updates: Vec<Update> = vec!();
    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let update_parse_result: Result<Update, UpdateParseError> = line.parse();

                match update_parse_result {
                    Ok(update) => updates.push(update),
                    Err(update_parse_error) => match update_parse_error {
                        UpdateParseError::InvalidUpdate => {
                            panic!("Invalid update: {}", line);
                        }
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules.iter() {
        if rules_map.contains_key(&rule.second) {
            rules_map.get_mut(&rule.second).unwrap().insert(rule.first);
        } else {
            rules_map.insert(rule.second, HashSet::from([rule.first]));
        }
    }

    let mut total: u32 = 0;
    for update in updates {
        total += get_update_value(update, &rules_map, true);
    }

    Some(total)
}

fn get_update_value(mut update: Update, rules_map: &HashMap<u32, HashSet<u32>>, first_iter: bool) -> u32 {
    let mut bad_indexes: Vec<usize> = vec!();
    let mut stop = false;
    for (index, page) in update.pages.iter().enumerate() {
        if rules_map.contains_key(page) {
            for (other_index, other_page) in update.pages.iter().skip(index + 1).enumerate() {
                if rules_map.get(page).unwrap().contains(&other_page) {
                    bad_indexes.push(other_index + index + 1);
                    stop = true;
                }
            }
        }

        if stop {
            break;
        }
    }

    if bad_indexes.len() > 0 {
        for bad_index in bad_indexes.iter() {
            let page = update.pages.remove(*bad_index);
            update.pages.insert(0, page);
        }
        return get_update_value(update, rules_map, false);
    }

    if first_iter {
        0
    } else {
        update.pages[update.pages.len() / 2]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
