use std::collections::{HashMap, HashSet};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.split(" ")
            .map(|num| num.parse::<u32>())
            .filter(|num| match num {
                Ok(_) => true,
                Err(_) => false
            })
            .map(|num| num.unwrap()));

    let mut list_1:Vec<u32> = vec!();
    let mut list_2:Vec<u32> = vec!();

    for mut row in rows {
        list_1.push(row.next().unwrap());
        list_2.push(row.next().unwrap());
    }

    list_1.sort();
    list_2.sort();

    let mut total_dif = 0;
    let mut i = 0;
    while i < list_1.len() {
        let dif = if list_1[i] > list_2[i] {
            list_1[i] - list_2[i]
        } else {
            list_2[i] - list_1[i]
        };
        total_dif = total_dif + dif;
        i = i + 1;
    }

    Some(total_dif)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.split(" ")
            .map(|num| num.parse::<u32>())
            .filter(|num| match num {
                Ok(_) => true,
                Err(_) => false
            })
            .map(|num| num.unwrap()));

    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2_map: HashMap<u32, u32> = HashMap::new();

    for mut row in rows {
        let list_1_num = row.next().unwrap();
        let list_2_num = row.next().unwrap();

        list_1.push(list_1_num);
        if list_2_map.contains_key(&list_2_num) {
            list_2_map.insert(list_2_num, list_2_map.get(&list_2_num).unwrap() + 1);
        } else {
            list_2_map.insert(list_2_num, 1);
        }
    }

    let mut total = 0;
    for item in list_1 {
        total = total + (item * list_2_map.get(&item).unwrap_or_else(|| &0));
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
