use std::cmp::min;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    let mut map_height = 0;
    let mut map_width = 0;
    for (i, line) in input.lines().enumerate() {
        map_height += 1;
        if line.is_empty() {
            break;
        }
        for (j, char) in line.chars().enumerate() {
            if i == 0 {
                map_width += 1;
            }
            if char == '.' {
                continue;
            }
            if let Some(coordinates) = antennas.get_mut(&char) {
                coordinates.push(Coordinate { x: i as i32, y: j as i32 });
            } else {
                antennas.insert(char, vec![Coordinate { x: i as i32, y: j as i32 }]);
            }
        }
    }

    let mut antinode_locations: HashSet<Coordinate> = HashSet::new();
    for (_, coordinates) in antennas.iter() {
        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let a = &coordinates[i];
                let b = &coordinates[j];

                let x_diff = a.x - b.x;
                let y_diff = a.y - b.y;

                let antinode_1 = Coordinate { x: a.x + x_diff, y: a.y + y_diff };
                if antinode_1.x >= 0 && antinode_1.x < map_height && antinode_1.y >= 0 && antinode_1.y < map_width {
                    antinode_locations.insert(antinode_1);
                }

                let antinode_2 = Coordinate { x: b.x - x_diff, y: b.y - y_diff };
                if antinode_2.x >= 0 && antinode_2.x < map_height && antinode_2.y >= 0 && antinode_2.y < map_width {
                    antinode_locations.insert(antinode_2);
                }
            }
        }
    }

    Some(antinode_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    let mut map_height = 0;
    let mut map_width = 0;
    for (i, line) in input.lines().enumerate() {
        map_height += 1;
        if line.is_empty() {
            break;
        }
        for (j, char) in line.chars().enumerate() {
            if i == 0 {
                map_width += 1;
            }
            if char == '.' {
                continue;
            }
            if let Some(coordinates) = antennas.get_mut(&char) {
                coordinates.push(Coordinate { x: i as i32, y: j as i32 });
            } else {
                antennas.insert(char, vec![Coordinate { x: i as i32, y: j as i32 }]);
            }
        }
    }

    let mut antinode_locations: HashSet<Coordinate> = HashSet::new();
    for (_, coordinates) in antennas.iter() {
        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let a = &coordinates[i];
                let b = &coordinates[j];
                let mut x_diff = a.x - b.x;
                let mut y_diff = a.y - b.y;

                for i in 2..min(x_diff, y_diff) {
                    if x_diff % i == 0 && y_diff % i == 0 {
                        x_diff /= i;
                        y_diff /= i;
                    }
                }

                antinode_locations.insert(*a);
                let mut multiplier = 1;
                let mut stop_adding = false;
                let mut stop_subtracting = false;
                loop {
                    if !stop_adding {
                        let antinode = Coordinate { x: a.x + (multiplier * x_diff), y: a.y + (multiplier * y_diff) };

                        if antinode.x >= 0 && antinode.x < map_height && antinode.y >= 0 && antinode.y < map_width {
                            antinode_locations.insert(antinode);
                        }
                        else {
                            stop_adding = true;
                        }
                    }

                    if !stop_subtracting {
                        let antinode = Coordinate { x: a.x - (multiplier * x_diff), y: a.y - (multiplier * y_diff) };

                        if antinode.x >= 0 && antinode.x < map_height && antinode.y >= 0 && antinode.y < map_width {
                            antinode_locations.insert(antinode);
                        }
                        else {
                            stop_subtracting = true;
                        }
                    }

                    if stop_adding && stop_subtracting {
                        break;
                    }

                    multiplier += 1;
                }
            }
        }
    }

    Some(antinode_locations.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
