use std::{iter, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let map = input.parse::<DiskMap>()?;
        let part1 = map.filesystem_checksum().to_string();
        let part2 = Some(map.defragment().to_string());
        Ok(DayResult { part1, part2 })
    }
}

struct DiskMap {
    map: Vec<u32>,
}

impl FromStr for DiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .chars()
            .map(|c| c.to_digit(10).ok_or("invalid digit".to_owned()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DiskMap { map })
    }
}

impl DiskMap {
    fn filesystem_checksum(&self) -> usize {
        let mut filesystem = FileSystem::from_disk_map(&self);

        while filesystem.has_free() {
            filesystem.move_one_left();
        }

        filesystem.checksum()
    }

    fn defragment(&self) -> usize {
        let mut filesystem = FileSystem::from_disk_map(&self);

        for i in (1..=filesystem.max_id).rev() {
            filesystem.move_block_left(i);
        }

        filesystem.checksum()
    }
}

struct FileSystem {
    data: Vec<Option<u32>>,
    free_space: usize,
    max_id: u32,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            data: Vec::new(),
            free_space: 0,
            max_id: 0,
        }
    }

    fn from_disk_map(map: &DiskMap) -> Self {
        let mut filesystem = FileSystem::new();
        for (i, &c) in map.map.iter().enumerate() {
            if i % 2 == 0 {
                filesystem.add(i as u32 / 2, c as usize);
            } else {
                filesystem.add_free(c as usize);
            }
        }
        filesystem
    }

    fn add(&mut self, id: u32, amount: usize) {
        self.data.extend(iter::repeat(Some(id)).take(amount));
        self.max_id = self.max_id.max(id);
    }

    fn add_free(&mut self, amount: usize) {
        self.data.extend(iter::repeat(None).take(amount));
        self.free_space += amount;
    }

    fn has_free(&self) -> bool {
        self.free_space > 0
    }

    fn move_one_left(&mut self) {
        let last = self.data.pop().unwrap();
        let first_free = self.data.iter().position(|&x| x.is_none()).unwrap();
        self.data[first_free] = last;
        self.free_space -= 1;
    }

    fn move_block_left(&mut self, id: u32) {
        let start_block = self.data.iter().position(|&x| x == Some(id)).unwrap();
        let end_block = start_block
            + self.data[start_block..]
                .iter()
                .position(|&x| x != Some(id))
                .unwrap_or(self.data.len() - start_block);

        let block_length = end_block - start_block;

        let mut free_index = 0;
        let mut free_block_size = 0;
        while free_index < start_block {
            if self.data[free_index].is_some() {
                free_index += 1;
                free_block_size = 0;
            } else if self.data[free_index].is_none() {
                free_block_size += 1;
                free_index += 1;
            }
            if free_block_size == block_length {
                for i in 0..block_length {
                    self.data[start_block + i] = None;
                    self.data[free_index - free_block_size + i] = Some(id);
                }
                break;
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        for (i, &x) in self.data.iter().enumerate() {
            checksum += i * (x.unwrap_or(0) as usize);
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "2333133121414131402".to_owned();
        let day = Instance;
        let result = day.run(input).unwrap();
        assert_eq!(result.part1, "1928");
        assert_eq!(result.part2, Some("2858".to_owned()));
    }
}
