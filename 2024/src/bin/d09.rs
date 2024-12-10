use std::{collections::VecDeque, fs};

type Id = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Range {
    start: usize,
    len: usize,
}

impl Range {
    fn end(&self) -> usize {
        self.start + self.len
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FileInfo {
    id: Id,
    range: Range,
}

type DiskMap = VecDeque<FileInfo>;

fn parse_input(contents: &str) -> DiskMap {
    let mut disk_map: DiskMap = VecDeque::with_capacity(contents.len());
    let mut id = 0;
    let mut is_file = true;
    let mut curr_loc = 0;
    for c in contents.trim().chars() {
        let n = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("Unexpected char {c}"),
        };
        if is_file {
            disk_map.push_back(FileInfo {
                id,
                range: Range {
                    start: curr_loc,
                    len: n,
                },
            });
            id += 1;
        }
        curr_loc += n;
        is_file = !is_file;
    }
    disk_map
}

fn next_gap_loc(disk_map: &DiskMap, curr_index: usize) -> (usize, Option<usize>) {
    for i in (curr_index + 1)..disk_map.len() {
        if disk_map[i].range.start > disk_map[i - 1].range.end() {
            return (
                i,
                Some(disk_map[i].range.start - disk_map[i - 1].range.end()),
            );
        }
    }
    (disk_map.len(), None)
}

fn checksum(disk_map: &DiskMap) -> usize {
    disk_map
        .iter()
        .map(|file_info| {
            (file_info.range.start..file_info.range.end())
                .map(|i| i * file_info.id)
                .sum::<usize>()
        })
        .sum()
}

fn compute_1(contents: &str) -> usize {
    let mut disk_map = parse_input(contents);
    let mut curr_index = 0;
    loop {
        let (next_insertion_index, maybe_gap_size) = next_gap_loc(&disk_map, curr_index);
        curr_index = next_insertion_index;
        if let Some(gap_size) = maybe_gap_size {
            let mut last_file_info = disk_map.pop_back().unwrap();
            let start = disk_map[curr_index - 1].range.end();
            if last_file_info.range.len > gap_size {
                disk_map.insert(
                    curr_index,
                    FileInfo {
                        id: last_file_info.id,
                        range: Range {
                            start,
                            len: gap_size,
                        },
                    },
                );
                last_file_info.range.len -= gap_size;
                disk_map.push_back(last_file_info);
            } else {
                last_file_info.range.start = start;
                disk_map.insert(curr_index, last_file_info);
            }
        } else {
            break;
        }
    }

    checksum(&disk_map)
}

fn compute_2(contents: &str) -> usize {
    let mut disk_map = parse_input(contents);

    for file_id in (0..(disk_map.len())).rev() {
        let id_within_disk_map = (0..disk_map.len())
            .find(|i| disk_map[*i].id == file_id)
            .unwrap();
        let mut file_info = disk_map[id_within_disk_map].clone();
        for i in 1..std::cmp::min(id_within_disk_map + 1, disk_map.len()) {
            if disk_map[i].range.start - disk_map[i - 1].range.end() >= file_info.range.len {
                file_info.range.start = disk_map[i - 1].range.end();
                if i == id_within_disk_map {
                    disk_map[id_within_disk_map] = file_info;
                } else {
                    disk_map.remove(id_within_disk_map);
                    disk_map.insert(i, file_info);
                }
                break;
            }
        }
    }

    checksum(&disk_map)
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d09.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(6359213660505, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(6381624803796, result);
    println!("part 2: {result}");
}
