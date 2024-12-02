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

    let mut list1:Vec<u32> = vec!();
    let mut list2:Vec<u32> = vec!();

    for mut row in rows {
        list1.push(row.next().unwrap());
        list2.push(row.next().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut total_dif = 0;
    let mut i = 0;
    while i < list1.len() {
        let dif = if list1[i] > list2[i] {
            list1[i] - list2[i]
        } else {
            list2[i] - list1[i]
        };
        total_dif = total_dif + dif;
        i = i + 1;
    }

    Some(total_dif)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
