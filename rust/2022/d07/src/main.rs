// https://adventofcode.com/2022/day/7

use scanf::sscanf;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

#[derive(Debug)]
struct DirectoryListing {
    files: HashMap<String, usize>,
    dirs: HashSet<String>,
}

struct Device {
    content: HashMap<String, DirectoryListing>,
}

impl Device {
    fn total_size(&self, dir: &str) -> usize {
        let dirlisting = self.content.get(dir).unwrap();
        dirlisting.files.values().sum::<usize>()
            + dirlisting
                .dirs
                .iter()
                .map(|d| {
                    let subdir = PathBuf::new()
                        .join(dir)
                        .join(d)
                        .to_string_lossy()
                        .to_string();
                    self.total_size(&subdir)
                })
                .sum::<usize>()
    }
}

#[derive(Debug)]
enum Output {
    CommandCd(String),
    CommandLs,
    ListingFile(String, usize),
    ListingDir(String),
}

impl From<Vec<String>> for Device {
    fn from(data: Vec<String>) -> Self {
        let output = data
            .iter()
            .map(|line| {
                if line.starts_with("$ cd ") {
                    Output::CommandCd(line[5..].to_string())
                } else if line.starts_with("$ ls") {
                    Output::CommandLs
                } else if line.chars().nth(0).unwrap().is_digit(10) {
                    let mut name: String = String::new();
                    let mut size: usize = 0;
                    sscanf!(line, "{} {}", size, name).unwrap();
                    Output::ListingFile(name, size)
                } else {
                    Output::ListingDir(line[4..].to_string())
                }
            })
            .collect::<Vec<_>>();

        let mut cwd = PathBuf::new();
        let mut content: HashMap<String, DirectoryListing> = HashMap::new();
        for out in output.into_iter() {
            match out {
                Output::CommandCd(cd) => {
                    if cd.starts_with('/') {
                        cwd = cd.into();
                    } else if cd.starts_with("..") {
                        cwd = cwd.parent().unwrap().into();
                    } else {
                        cwd = cwd.join(cd);
                    }
                }
                Output::CommandLs => {}
                Output::ListingFile(name, size) => {
                    let listing =
                        content
                            .entry(cwd.to_string_lossy().into())
                            .or_insert(DirectoryListing {
                                files: HashMap::new(),
                                dirs: HashSet::new(),
                            });
                    listing.files.insert(name, size);
                }
                Output::ListingDir(name) => {
                    let listing =
                        content
                            .entry(cwd.to_string_lossy().into())
                            .or_insert(DirectoryListing {
                                files: HashMap::new(),
                                dirs: HashSet::new(),
                            });
                    listing.dirs.insert(name);
                }
            }
        }

        Self { content }
    }
}

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().filter_map(Result::ok).collect()
}

fn part_1(device: &Device) -> usize {
    device
        .content
        .keys()
        .filter_map(|d| {
            let size = device.total_size(d);
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(device: &Device) -> usize {
    let root_size = device.total_size("/");
    let free_space = 70000000 - root_size;
    let to_free = 30000000 - free_space;
    let mut size_to_name = device
        .content
        .keys()
        .map(|d| {
            let size = device.total_size(d);
            size
        })
        .collect::<Vec<_>>();
    size_to_name.sort();
    let idx = size_to_name.partition_point(|e| e < &to_free);
    size_to_name[idx]
}

fn main() {
    let device = read(File::open("input.txt").unwrap()).into();
    let p1 = part_1(&device);
    println!("Part 1: {}", p1);
    let p2 = part_2(&device);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let device: Device = read(File::open("example1.txt").unwrap()).into();
        assert_eq!(device.total_size("/a/e"), 584);
        assert_eq!(device.total_size("/a"), 94853);
        assert_eq!(device.total_size("/d"), 24933642);
        assert_eq!(device.total_size("/"), 48381165);

        assert_eq!(part_1(&device), 95437);
        assert_eq!(part_2(&device), 24933642);
    }
}
