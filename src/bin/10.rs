use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map:Vec<Vec<u32>> = vec!();
    let mut trailheads: Vec<Coordinate> = vec!();

    for (x, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut map_line:Vec<u32> = vec!();
        let heights_enumeration = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate();
        for (y, height) in heights_enumeration {
            map_line.push(height);
            if height == 0 {
                trailheads.push(Coordinate { x, y });
            }
        }
        map.push(map_line);
    }

    let mut total_score = 0;
    for trailhead in trailheads {
        let paths = get_paths(trailhead, &map);
        total_score += paths.len() as u32;
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map:Vec<Vec<u32>> = vec!();
    let mut trailheads: Vec<Coordinate> = vec!();

    for (x, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut map_line:Vec<u32> = vec!();
        let heights_enumeration = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate();
        for (y, height) in heights_enumeration {
            map_line.push(height);
            if height == 0 {
                trailheads.push(Coordinate { x, y });
            }
        }
        map.push(map_line);
    }

    let mut total_rating = 0;
    for trailhead in trailheads {
        total_rating += get_rating(&trailhead, &map);
    }

    Some(total_rating)
}

// TODO if I make map a Vec<Vec<SomethingElse>> I can keep track of visited nodes and avoid the HashSet stuff
fn get_paths(starting_coordinate: Coordinate, map: &Vec<Vec<u32>>) -> HashSet<Coordinate> {
    let mut paths_set = HashSet::new();

    if map[starting_coordinate.x][starting_coordinate.y] == 9 {
        paths_set.insert(starting_coordinate);
        return paths_set;
    }

    if starting_coordinate.x + 1 < map.len() && map[starting_coordinate.x+1][starting_coordinate.y] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        let paths = get_paths(Coordinate { x: starting_coordinate.x+1, y: starting_coordinate.y }, &map);
        paths_set.extend(paths);
    }
    if starting_coordinate.y + 1 < map[starting_coordinate.x].len() && map[starting_coordinate.x][starting_coordinate.y+1] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        let paths = get_paths(Coordinate { x: starting_coordinate.x, y: starting_coordinate.y+1 }, &map);
        paths_set.extend(paths);
    }
    if starting_coordinate.x > 0 && map[starting_coordinate.x-1][starting_coordinate.y] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        let paths = get_paths(Coordinate { x: starting_coordinate.x-1, y: starting_coordinate.y }, &map);
        paths_set.extend(paths);
    }
    if starting_coordinate.y > 0 && map[starting_coordinate.x][starting_coordinate.y-1] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        let paths = get_paths(Coordinate { x: starting_coordinate.x, y: starting_coordinate.y-1 }, &map);
        paths_set.extend(paths);
    }

    paths_set
}

fn get_rating(starting_coordinate: &Coordinate, map: &Vec<Vec<u32>>) -> u32 {
    if map[starting_coordinate.x][starting_coordinate.y] == 9 {
        return 1
    }

    let mut total_rating = 0;
    if starting_coordinate.x + 1 < map.len() && map[starting_coordinate.x+1][starting_coordinate.y] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        total_rating += get_rating(&Coordinate { x: starting_coordinate.x+1, y: starting_coordinate.y }, &map);
    }
    if starting_coordinate.y + 1 < map[starting_coordinate.x].len() && map[starting_coordinate.x][starting_coordinate.y+1] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        total_rating += get_rating(&Coordinate { x: starting_coordinate.x, y: starting_coordinate.y+1 }, &map);
    }
    if starting_coordinate.x > 0 && map[starting_coordinate.x-1][starting_coordinate.y] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        total_rating += get_rating(&Coordinate { x: starting_coordinate.x-1, y: starting_coordinate.y }, &map);
    }
    if starting_coordinate.y > 0 && map[starting_coordinate.x][starting_coordinate.y-1] == map[starting_coordinate.x][starting_coordinate.y] + 1 {
        total_rating += get_rating(&Coordinate { x: starting_coordinate.x, y: starting_coordinate.y-1 }, &map);
    }

    total_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
