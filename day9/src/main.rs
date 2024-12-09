use itertools::Itertools;
use std::cmp::min;
use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]
enum BlockType {
    Occupied(u64), // File index
    Free,
}

#[derive(Clone, Copy, Debug)]
struct Block {
    block_type: BlockType,
    block_size: u64,
    starting_index: u64,
}

fn part_one(disk_map: &String) -> u64 {
    let mut free_blocks: Vec<u64> = Vec::new();
    let mut occupied_blocks: Vec<(u64, u64)> = Vec::new();
    let mut seen_blocks_count: u64 = 0;
    let mut checksum: u64 = 0;
    for (i, block_size) in disk_map.chars().enumerate() {
        let block_size: u64 = block_size.to_digit(10).unwrap() as u64;
        let found_range = seen_blocks_count..seen_blocks_count + block_size;
        if i % 2 == 0 {
            let file_index = i / 2;
            occupied_blocks.extend(
                found_range
                    .map(|position| (position, file_index as u64))
                    .collect::<Vec<_>>(),
            );
        } else {
            free_blocks.extend(found_range);
        }
        seen_blocks_count += block_size;
    }

    let occupied_back_iter = occupied_blocks.iter().rev();
    for zip_element in occupied_back_iter.zip_longest(free_blocks) {
        match zip_element {
            itertools::EitherOrBoth::Left(&(occupied_block, occupied_block_file_index)) => {
                checksum += occupied_block * (occupied_block_file_index as u64);
            }
            itertools::EitherOrBoth::Both(
                &(occupied_block, occupied_block_file_index),
                free_block,
            ) => checksum += min(free_block, occupied_block) * occupied_block_file_index,
            _ => {}
        }
    }
    checksum
}

fn range_sum(start: u64, end: u64) -> u64 {
    ((start + end) * (end - start + 1)) / 2
}

fn part_two(disk_map: &String) -> u64 {
    let mut blocks: Vec<Block> = Vec::new();
    let mut seen_indices = 0;
    for (i, block_size) in disk_map.chars().enumerate() {
        let block_size = block_size.to_digit(10).unwrap() as u64;
        blocks.push(Block {
            block_type: match i % 2 {
                0 => BlockType::Occupied((i / 2) as u64),
                1 => BlockType::Free,
                _ => panic!(),
            },
            block_size,
            starting_index: seen_indices,
        });
        seen_indices += block_size;
    }
    let mut checksum: u64 = 0;
    for occupied_block in blocks
        .clone()
        .iter()
        .rev()
        .filter(|block| block.block_type != BlockType::Free)
    {
        let mut found_match = false;
        if let Some(first_suitable_block) = blocks
            .iter_mut()
            .filter(|block| block.block_type == BlockType::Free)
            .filter(|block| block.block_size >= occupied_block.block_size)
            .next()
        {
            if first_suitable_block.starting_index < occupied_block.starting_index {
                found_match = true;
                match occupied_block.block_type {
                    BlockType::Occupied(file_index) => {
                        checksum += file_index
                            * range_sum(
                                first_suitable_block.starting_index,
                                first_suitable_block.starting_index + occupied_block.block_size - 1,
                            );
                        first_suitable_block.block_size -= occupied_block.block_size;
                        first_suitable_block.starting_index += occupied_block.block_size;
                    }
                    _ => panic!(),
                }
            }
        }
        if !found_match {
            match occupied_block.block_type {
                BlockType::Occupied(file_index) => {
                    checksum += file_index
                        * range_sum(
                            occupied_block.starting_index,
                            occupied_block.starting_index + occupied_block.block_size - 1,
                        )
                }
                _ => panic!(),
            }
        }
    }
    checksum
}

fn main() {
    let disk_map: String = io::stdin().lines().next().unwrap().unwrap();
    println!("{} {}", part_one(&disk_map), part_two(&disk_map));
}
