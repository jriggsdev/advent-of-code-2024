use std::fmt;
use std::fmt::Formatter;

advent_of_code::solution!(9);

#[derive(Debug)]
enum BlockType {
    File(u32),
    Space
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            BlockType::File(val) => write!(f, "{val}"),
            BlockType::Space => write!(f, "."),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.lines().next().unwrap();

    let mut blocks: Vec<BlockType> = vec!();
    let mut file_block = true;
    let mut file_id = 0;
    for char in input.chars() {
        let space_val = char.to_digit(10).unwrap();
        if file_block {
            for j in 0..space_val {
                blocks.push(BlockType::File(file_id));
            }
            file_id += 1;
        } else {
            for j in 0..space_val {
                blocks.push(BlockType::Space);
            }
        }

        file_block = !file_block;
    }

    let mut checksum: u64 = 0;
    let mut multiplier: u64 = 0;
    let mut j = blocks.len() - 1;
    for i in 0..blocks.len() {
        match blocks[i] {
            BlockType::File(val) => {
                checksum += val as u64 * multiplier;
            }
            BlockType::Space => {
                loop {
                    match blocks[j] {
                        BlockType::File(val) => {
                            checksum += val as u64 * multiplier;
                            j -= 1;
                            break;
                        },
                        BlockType::Space => {
                            j -= 1;
                        }
                    }
                    if i >= j {
                        break;
                    }
                }
            }
        }

        multiplier += 1;
        if i >= j {
            break;
        }
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.lines().next().unwrap();
    let mut blocks: Vec<BlockType> = vec!();
    let mut file_block = true;
    let mut file_id = 0;
    for char in input.chars() {
        let space_val = char.to_digit(10).unwrap();
        if file_block {
            for j in 0..space_val {
                blocks.push(BlockType::File(file_id));
            }
            file_id += 1;
        } else {
            for j in 0..space_val {
                blocks.push(BlockType::Space);
            }
        }

        file_block = !file_block;
    }

    let mut i = blocks.len() - 1;
    while i > 1 {
        match blocks[i] {
            BlockType::File(file_id) => {
                let mut moved = false;
                let mut file_size = 1;
                loop {
                    if let BlockType::File(other_file_id) = blocks[i - file_size] {
                        if file_id == other_file_id {
                            file_size += 1;

                            if i - file_size == 0 {
                                break;
                            }
                            continue;
                        }
                    }
                    break;
                }

                for j in 0..=i-file_size {
                    if let BlockType::Space = blocks[j] {
                        let mut available_space = 1;
                        loop {
                           if let BlockType::Space = blocks[j + available_space] {
                                available_space += 1;
                            } else {
                                break;
                            }
                        }

                        if available_space >= file_size {
                            moved = true;
                            for k in j..j+file_size {
                                blocks[k] = BlockType::File(file_id);
                            }

                            for k in i-file_size+1..=i {
                                blocks[k] = BlockType::Space;
                            }

                            break;
                        }
                    }
                }

                i -= file_size
            }
            BlockType::Space => {
                i -= 1;
            }
        }
    }

    let mut checksum = 0;
    for i in 0..blocks.len() {
        match blocks[i] {
            BlockType::File(val) => {
                checksum += val as u64 * i as u64;
            }
            BlockType::Space => {
                continue;
            }
        }
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
