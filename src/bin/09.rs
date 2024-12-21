use std::io::Write;
use std::io::stdout;
use itertools::Either::{Left, Right};
use itertools::{repeat_n, Itertools};
use crate::DiskPart::{File, Space};

advent_of_code::solution!(9);

fn parse(s: &str) -> (Vec<u32>, Vec<u32>) {
    s
    .chars()
    .flat_map(|c| c.to_digit(10))
    .enumerate()
    .partition_map(|(i, e)| if i % 2 == 0 { Left(e) } else { Right(e) })
}

struct FilePart {
    id: usize,
    length: u32,
}

fn compact_disk(files: &[u32], spaces: &[u32]) -> Vec<FilePart> {
    let mut compact_disk = Vec::new();
    let mut files_to_move = Vec::from(files);
    let mut offset = 0;
    while files_to_move.len() > offset {
        compact_disk.push(FilePart {id: offset, length: files_to_move[offset]});
        offset += 1;
        // FILL space
        if let Some(space) = spaces.get(offset - 1) {
            let mut space = *space;
            while space > 0 && files_to_move.len() > offset {
                if let Some(last) = files_to_move.pop() {
                    if last <= space {
                        // take WHOLE file from rear, filling PART of the space in front
                        space -= last;
                        compact_disk.push(FilePart {id: files_to_move.len(), length: last});
                    } else {
                        // take PART of the rear file, filling the WHOLE space in front
                        compact_disk.push(FilePart {id: files_to_move.len(), length: space});
                        files_to_move.push(last - space);
                        space = 0;
                    }
                }
            }
        }
    };
    compact_disk
}

// sum from to not inclusive
fn sum_from_to(from: usize, to: usize) -> usize {
    ((to - from) * (to + from - 1)) / 2
}

fn checksum<I>(disk: I) -> usize
where I: IntoIterator<Item = DiskPart>
{
    disk.into_iter().fold((0, 0), |(i, s), p| {
            match p {
                File(f) => {
                    let l = f.length as usize;
                    let size = sum_from_to(i, i + l);
                    (i + l, s + f.id * size)
                },
                Space(space) => (i + space as usize, s),
            }
        })
        .1
}

pub fn part_one(input: &str) -> Option<usize> {
    let (files, spaces) = parse(input);
    let compact = compact_disk(&files, &spaces);
    // debug_disk(&compact);
    Some(checksum(compact.into_iter().map(|p| File(p))))
}

enum DiskPart {
    File(FilePart),
    Space(u32),
}

fn new_file(file_id: usize, length: u32) -> DiskPart {
    File(FilePart {id: file_id, length})
}

#[allow(dead_code)]
fn debug_disk(disk: &[DiskPart]) {
    let mut out = stdout().lock();
    write!(out, "MINE:  ").unwrap();
    for part in disk {
        match part {
            File(f) => {
                assert!(f.id < 10, "file part ID too large to debug");
                write!(out, "{}", repeat_n(char::from_digit(f.id as u32, 10).unwrap(), f.length as usize).collect::<String>()).unwrap();
            }
            Space(s) => {
                write!(out, "{}", repeat_n('.', *s as usize).collect::<String>()).unwrap();
            }
        }
    }
    writeln!(out).unwrap();
    out.flush().unwrap();
}

fn parse2(s: &str) -> Vec<DiskPart> {
    s
    .chars()
    .flat_map(|c| c.to_digit(10))
    .enumerate()
    .map(|(i, e)| if i % 2 == 0 { new_file(i / 2, e) } else { Space(e) })
    .collect()
}

fn compact_disk2(files: Vec<DiskPart>) -> Vec<DiskPart> {
    let mut files = files;
    // get the largest file index
    // let last_file = files.iter().rev().find_map(|p| match p { File(i, _) => Some(*i), _ => None }).unwrap();
    let last_file = files.len() / 2;

    for original_id in (0..=last_file).rev() {
        // println!("next: {original_id}");
        let (current_pos, length) = files.iter()
            .enumerate()
            .rev()
            .find_map(|(pos, p)| match p { File(p) => if p.id == original_id {Some((pos, p.length))} else {None}, _ => None })
            .unwrap();
        if let Some((space_pos, space)) = files
            .iter()
            .take(current_pos)
            .enumerate()
            .find_map(|(i, p)| match p { Space(s) => if *s >= length {Some((i, *s))} else { None }, _ => None })
        {
            // println!("move: file {original_id} length {length} from position {current_pos} to space at {space_pos} of size {space}");
            files[current_pos] = Space(length);
            if space == length {
                files[space_pos] = new_file(original_id, length);
            } else {
                files[space_pos] = Space(space - length);
                files.insert(space_pos, new_file(original_id, length));
            }
            // debug_disk(&files);
        }
    }
    files
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk = parse2(input);
    // debug_disk(&disk);
    let compact = compact_disk2(disk);
    // debug_disk(&compact);
    Some(checksum(compact))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        // 0..111....22222
        assert_eq!(parse("12345"), (vec![1, 3, 5], vec![2, 4]));
    }

    #[test]
    fn test_sum() {
        for i in 0..10 {
            for j in i+1..10 {
                assert_eq!(sum_from_to(i, j), (i..j).sum::<usize>(), "{i}..{j}");
            }
        }
    }

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
