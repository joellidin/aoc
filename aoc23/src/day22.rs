use core::fmt;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Block {
    start: Position,
    end: Position,
    id: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
pub struct ParseBlockError;
impl fmt::Display for ParseBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseBlockError {}

impl FromStr for Block {
    type Err = ParseBlockError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_pos_str, second_pos_str) = s.split_once('~').ok_or(ParseBlockError)?;
        let left_bottom_pos = first_pos_str
            .parse::<Position>()
            .map_err(|_| ParseBlockError)?;
        let right_top_pos = second_pos_str
            .parse::<Position>()
            .map_err(|_| ParseBlockError)?;
        Ok(Self {
            start: left_bottom_pos,
            end: right_top_pos,
            id: None,
        })
    }
}

#[derive(Debug)]
pub enum ParsePositionError {
    ParseIntError(std::num::ParseIntError),
    SplitError { e: &'static str },
}

impl fmt::Display for ParsePositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParsePositionError {}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(ParsePositionError::SplitError {
                e: "Could not split into three number",
            });
        }
        let x = parts.first().ok_or(ParsePositionError::SplitError {
            e: "Could not get x position",
        })?;
        let y = parts.get(1).ok_or(ParsePositionError::SplitError {
            e: "Could not get y position",
        })?;
        let z = parts.get(2).ok_or(ParsePositionError::SplitError {
            e: "Could not get z position",
        })?;

        Ok(Position {
            x: x.parse().map_err(ParsePositionError::ParseIntError)?,
            y: y.parse().map_err(ParsePositionError::ParseIntError)?,
            z: z.parse().map_err(ParsePositionError::ParseIntError)?,
        })
    }
}

impl Block {
    fn fall(&mut self, other_blocks: &[Block]) -> HashSet<usize> {
        let mut moved_blocks = HashSet::new();
        loop {
            let mut new_block = *self;
            new_block.start.z -= 1;
            new_block.end.z -= 1;
            if new_block.is_colliding(other_blocks) || new_block.start.z == 0 {
                break;
            }
            *self = new_block;
            moved_blocks.insert(self.id.unwrap());
        }
        moved_blocks
    }

    fn is_colliding(&self, other_blocks: &[Block]) -> bool {
        for other in other_blocks {
            if self != other && self.overlaps(other) {
                return true;
            }
        }
        false
    }

    fn overlaps(&self, other: &Block) -> bool {
        let overlap_x = self.start.x <= other.end.x && self.end.x >= other.start.x;
        let overlap_y = self.start.y <= other.end.y && self.end.y >= other.start.y;
        let overlap_z = self.start.z <= other.end.z && self.end.z >= other.start.z;

        overlap_x && overlap_y && overlap_z
    }

    fn blocks_fall_on_disintegration(&self, all_blocks: &[Block]) -> bool {
        for other in all_blocks {
            if self != other && self.is_supporting(other) {
                // Check if 'other' has support from blocks other than 'self'
                if all_blocks
                    .iter()
                    .any(|b| b != self && b != other && b.is_supporting(other))
                {
                    continue;
                }
                return true; // 'other' is solely supported by 'self'
            }
        }
        false
    }

    fn is_supporting(&self, other: &Block) -> bool {
        // This method checks if 'self' supports 'other' partially or fully
        let overlap_x = self.start.x <= other.end.x && self.end.x >= other.start.x;
        let overlap_y = self.start.y <= other.end.y && self.end.y >= other.start.y;
        let supports_z = self.end.z == other.start.z - 1;

        overlap_x && overlap_y && supports_z
    }

    fn count_falling_blocks_on_disintegration(&self, blocks: &[Block]) -> usize {
        let mut new_blocks = blocks.to_vec();
        let delete_id = new_blocks.iter().position(|b| self == b).unwrap();
        new_blocks.remove(delete_id);
        fall_to_ground(&mut new_blocks)
    }
}

fn fall_to_ground(blocks: &mut Vec<Block>) -> usize {
    // Assuming the blocks are sorted in ascending z-order
    let mut moved_blocks = HashSet::new();
    for i in 0..blocks.len() {
        let (other_blocks, current_block) = blocks.split_at_mut(i);
        moved_blocks.extend(current_block[0].fall(other_blocks));
    }
    moved_blocks.len()
}

pub fn generator(input: &str) -> Vec<Block> {
    let mut blocks = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut block = l.parse::<Block>().expect("Wrong input format");
            block.id = Some(i);
            block
        })
        .collect::<Vec<Block>>();
    blocks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    fall_to_ground(&mut blocks);
    blocks
}

pub fn part_1(input: &[Block]) -> usize {
    input
        .iter()
        .filter(|b| !b.blocks_fall_on_disintegration(input))
        .count()
}

pub fn part_2(input: &[Block]) -> usize {
    input
        .iter()
        .filter(|b| b.blocks_fall_on_disintegration(input))
        .map(|b| b.count_falling_blocks_on_disintegration(input))
        .sum()
}
