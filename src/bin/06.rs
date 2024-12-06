advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Clone)]
enum Space{
    Unvisited,
    Visited(Direction),
    Obstacle,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum StepError {
    Blocked,
    OutOfMap,
    CaughtInLoop,
}

#[derive(Debug, Clone)]
struct Map {
    spaces: Vec<Vec<Space>>,
}

#[derive(Debug)]
struct Guard {
    position: (u32, u32),
    direction: Direction,
}

impl Guard {
    fn step(&mut self, map: &mut Map) -> Result<(), StepError> {
        let mut new_x: i32 = self.position.0 as i32;
        let mut new_y: i32 = self.position.1 as i32;
        match self.direction {
            Direction::North => {
                new_x -= 1;
            },
            Direction::East => {
                new_y += 1;
            },
            Direction:: South => {
                new_x += 1;
            },
            Direction:: West => {
                new_y -= 1;
            }
        }
        if new_x < 0 || new_x >= map.spaces.len() as i32 || new_y < 0 || new_y >= map.spaces[0].len() as i32 {
            return Err(StepError::OutOfMap);
        }

        if map.spaces[new_x as usize][new_y as usize] == Space::Obstacle {
            return Err(StepError::Blocked);
        }

        if map.spaces[new_x as usize][new_y as usize] == Space::Visited(self.direction.clone()) {
            return Err(StepError::CaughtInLoop)
        }

        self.position.0 = new_x as u32;
        self.position.1 = new_y as u32;
        map.spaces[self.position.0 as usize][self.position.1 as usize] = Space::Visited(self.direction.clone());
        Ok(())
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }
}

fn parse_input(input: &str) -> (Map, Guard) {
    let mut map: Map = Map { spaces: vec!() };
    let mut guard: Guard = Guard { position: (0, 0), direction: Direction::North };

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut current_map_line: Vec<Space> = vec!();

        for c in line.chars() {
            match c {
                '#' => {
                    current_map_line.push(Space::Obstacle);
                },
                '.' => {
                    current_map_line.push(Space::Unvisited);
                },
                '^' => {
                    guard.position.0 = map.spaces.len() as u32;
                    guard.position.1 = current_map_line.len() as u32;
                    current_map_line.push(Space::Visited(Direction::North));
                },
                _ => {
                    panic!("Unexpected space in map.")
                }
            }
        }

        map.spaces.push(current_map_line)
    }

    (map, guard)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, mut guard) = parse_input(input);

    loop {
        let step_result = guard.step(&mut map);
        match step_result {
            Ok(_) => {
            },
            Err(StepError::Blocked) => {
                guard.turn();
            },
            Err(StepError::OutOfMap) => {
                break;
            },
            Err(StepError::CaughtInLoop) => {
                panic!("Guard caught in loop.");
            }
        }
    }

    let spaces_visited = map.spaces.iter()
        .flat_map(|line| line)
        .filter(|space| match **space { Space::Visited(_) => true, _ => false })
        .count() as u32;

    Some(spaces_visited)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, mut guard) = parse_input(input);

    let mut initial_run_map = map.clone();
    let guard_starting_point = guard.position;
    loop {
        let step_result = guard.step(&mut initial_run_map);
        match step_result {
            Ok(_) => {
            },
            Err(StepError::Blocked) => {
                guard.turn();
            },
            Err(StepError::OutOfMap) => {
                break;
            },
            Err(StepError::CaughtInLoop) => {
                panic!("Guard caught in loop.");
            }
        }
    }

    let mut total = 0;
    for i in 0..initial_run_map.spaces.len() {
        for j in 0..initial_run_map.spaces[i].len() {
            match &initial_run_map.spaces[i][j] {
                Space::Visited(_) => {
                    if guard_starting_point != (i as u32, j as u32) {
                        let mut new_map = map.clone();
                        new_map.spaces[i][j] = Space::Obstacle;
                        guard.position = guard_starting_point;
                        guard.direction = Direction::North;

                        loop {
                            let step_result = guard.step(&mut new_map);
                            match step_result {
                                Ok(_) => {
                                },
                                Err(StepError::Blocked) => {
                                    guard.turn();
                                },
                                Err(StepError::OutOfMap) => {
                                    break;
                                },
                                Err(StepError::CaughtInLoop) => {
                                    total += 1;
                                    break;
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
