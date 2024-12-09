// https://adventofcode.com/2024/day/9

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Copy, Clone)]
enum Item {
    File(usize, usize),
    Free(usize),
}

#[derive(Debug, Clone)]
struct Filesystem {
    items: VecDeque<Item>,
}

impl Filesystem {
    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut block_idx = 0;
        self.items.iter().for_each(|item| match item {
            Item::File(id, mut size) => {
                while size > 0 {
                    checksum += block_idx * id;
                    block_idx += 1;
                    size -= 1;
                }
            }
            Item::Free(size) => {
                block_idx += size;
            }
        });
        checksum
    }

    fn defrag(mut self) -> Self {
        let mut items = VecDeque::new();

        let mut back = self.items.pop_back().unwrap();
        while let Some(mut item) = self.items.pop_front() {
            assert!(back.is_file());
            if item.is_file() {
                items.push_back(item);
            } else {
                while item.size() > 0 {
                    if back.size() == 0 {
                        let Some(new_back) = self.items.pop_back() else {
                            break;
                        };
                        assert!(new_back.is_free());
                        let Some(new_back) = self.items.pop_back() else {
                            break;
                        };
                        assert!(new_back.is_file());
                        back = new_back;
                    }
                    let consume_size = item.size().min(back.size());
                    items.push_back(Item::Free(0));
                    items.push_back(Item::File(back.id(), consume_size));
                    item.consume(consume_size);
                    back.consume(consume_size);
                }
            }
        }
        if back.size() > 0 {
            items.push_back(Item::Free(0));
            items.push_back(back);
        }

        Self { items }
    }

    fn idx_of_file(&self, id: usize) -> Option<usize> {
        self.items
            .iter()
            .position(|item| item.is_file() && item.id() == id)
    }

    fn idx_of_free_before(&self, min: usize, idx: usize) -> Option<usize> {
        (0..idx).find(|&i| self.items[i].is_free() && self.items[i].size() >= min)
    }

    fn defrag2(&mut self) {
        let mut id = self.items[self.items.len() - 1].id();
        while id > 0 {
            let idx_file = self.idx_of_file(id).unwrap();
            if let Some(idx_free) = self.idx_of_free_before(self.items[idx_file].size(), idx_file) {
                assert!(idx_free < idx_file);
                let file = self.items.remove(idx_file).unwrap();
                let free_after = self
                    .items
                    .remove(idx_file)
                    .map(|item| item.size())
                    .unwrap_or(0);
                self.items[idx_file - 1].increase(file.size() + free_after);
                self.items[idx_free].consume(file.size());
                self.items.insert(idx_free, file);
                self.items.insert(idx_free, Item::Free(0));
            };

            id -= 1;
        }
    }
}

impl Item {
    fn size(&self) -> usize {
        match self {
            Item::File(_, size) => *size,
            Item::Free(size) => *size,
        }
    }

    fn consume(&mut self, n: usize) {
        assert!(self.size() >= n);
        match self {
            Item::File(_, size) => *size -= n,
            Item::Free(size) => *size -= n,
        }
    }

    fn increase(&mut self, n: usize) {
        match self {
            Item::File(_, _) => panic!("Cannot increase file size"),
            Item::Free(size) => *size += n,
        }
    }

    fn is_file(&self) -> bool {
        matches!(self, Item::File(_, _))
    }

    fn is_free(&self) -> bool {
        matches!(self, Item::Free(_))
    }

    fn id(&self) -> usize {
        match self {
            Item::File(id, _) => *id,
            _ => panic!("Item is not a file"),
        }
    }
}

fn read<R: Read>(io: R) -> Vec<Item> {
    let br = BufReader::new(io);
    let line = br.lines().next().unwrap().unwrap();

    let mut id = 0;
    let mut items = Vec::new();

    for (i, c) in line.chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            items.push(Item::File(id, num));
            id += 1;
        } else {
            items.push(Item::Free(num));
        }
    }

    items
}

fn part_1(input: &[Item]) -> usize {
    let fs = Filesystem {
        items: input.iter().copied().collect(),
    };
    let defragged_fs = fs.defrag();
    defragged_fs.checksum()
}

fn part_2(input: &[Item]) -> usize {
    let mut fs = Filesystem {
        items: input.iter().copied().collect(),
    };
    fs.defrag2();
    fs.checksum()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input), 1928);
        assert_eq!(part_2(&input), 2858);
    }
}
