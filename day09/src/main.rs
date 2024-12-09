use std::{collections::HashMap, fs};

type File = (usize, usize);

fn read_lines() -> Vec<char> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let disk_map: Vec<char> = contents.chars().collect();

    return disk_map;
}

fn get_blocks(disk_map: &Vec<char>) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let mut current_id = 0;
    for i in 0..disk_map.len() {
        for _ in 0..disk_map[i].to_digit(10).unwrap() {
            if i % 2 == 0 {
                blocks.push(current_id.to_string());
            } else {
                blocks.push(".".to_string());
            }
        }
        if i % 2 == 0 {
            current_id += 1;
        }
    }
    return blocks;
}

fn defragment(blocks: &mut Vec<String>) {
    let mut last_free_position: usize = 0;
    for i in (0..blocks.len()).rev() {
        let mut found_free_space = false;
        for j in last_free_position..i {
            if blocks[j] == "." {
                blocks[j] = blocks[i].to_string();
                blocks[i] = ".".to_string();
                last_free_position = j;
                found_free_space = true;
                break;
            }
        }
        if !found_free_space {
            break;
        }
    }
}

fn part_1(disk_map: &Vec<char>) {
    let mut blocks = get_blocks(disk_map);
    defragment(&mut blocks);
    let checksum = blocks
        .iter()
        .enumerate()
        .map(|(idx, number)| match number.as_ref() {
            "." => 0,
            _ => number.parse::<usize>().unwrap() * idx,
        })
        .sum::<usize>();
    println!("Part 1: {:?}", checksum);
}

fn get_files(disk_map: &Vec<char>) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let mut idx = 0;
    for i in 0..disk_map.len() {
        let digit: usize = disk_map[i].to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push((idx as usize, digit));
        }
        idx += digit;
    }
    return files;
}

fn get_free_spaces(disk_map: &Vec<char>) -> HashMap<usize, Vec<usize>> {
    let mut free_spaces: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut idx = 0;
    for i in 0..disk_map.len() {
        let digit = disk_map[i].to_digit(10).unwrap() as usize;
        if i % 2 != 0 {
            free_spaces.entry(digit).or_default().push(idx);
        }
        idx += digit;
    }
    return free_spaces;
}

fn find_free_space(
    free_spaces: &mut HashMap<usize, Vec<usize>>,
    before: usize,
    size: usize,
) -> Option<(usize, usize)> {
    for free_space in free_spaces {
        if free_space.0 < &size {
            continue;
        }
        for i in 0..free_space.1.len() {
            let space = free_space.1[i];
            if space < before {
                let new_space = free_space.1.remove(i);
                return Some((new_space, free_space.0 - &size));
            }
        }
    }
    return None;
}

fn defragment_with_files(
    blocks: &mut Vec<String>,
    files: &mut Vec<File>,
    free_spaces: &mut HashMap<usize, Vec<usize>>,
) {
    for i in (0..files.len()).rev() {
        let file_pos = files[i].0;
        let file_size = files[i].1;
        if let Some((free_space_pos, space_left)) =
            find_free_space(free_spaces, files[i].0, file_size)
        {
            for j in 0..file_size {
                blocks[free_space_pos + j] = i.to_string();
                blocks[file_pos + j] = ".".to_string();
            }
            files[i].0 = free_space_pos;
            if space_left > 0 {
                free_spaces
                    .entry(space_left)
                    .or_default()
                    .push(free_space_pos + file_size);
            }
        }
    }
}

fn part_2(disk_map: &Vec<char>) {
    let mut blocks = get_blocks(disk_map);
    let mut files = get_files(disk_map);
    let mut free_spaces = get_free_spaces(disk_map);
    defragment_with_files(&mut blocks, &mut files, &mut free_spaces);
    let checksum = blocks
        .iter()
        .enumerate()
        .map(|(idx, number)| match number.as_ref() {
            "." => 0,
            _ => number.parse::<usize>().unwrap() * idx,
        })
        .sum::<usize>();
    println!("Part 2: {:?}", checksum);
}

fn main() {
    let disk_map = read_lines();
    part_1(&disk_map);
    part_2(&disk_map);
}
