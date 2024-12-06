use std::{collections::HashSet, fs};

use indicatif::ProgressBar;

type Coord = (i32, i32);

fn read_lines() -> Vec<Vec<char>> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let map = contents
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    return map;
}

fn is_in_bounds(current_position: Coord, map_size: (i32, i32)) -> bool {
    if current_position.0 < 0
        || current_position.1 < 0
        || current_position.0 >= map_size.0
        || current_position.1 >= map_size.1
    {
        return false;
    }
    return true;
}

fn add(x: &Coord, y: &Coord) -> Coord {
    return (x.0 + y.0, x.1 + y.1);
}

fn get_current_position(map: &Vec<Vec<char>>) -> Coord {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                return (j as i32, i as i32);
            }
        }
    }
    return (0, 0);
}

fn part_1() {
    let map = read_lines();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut current_position = get_current_position(&map);

    let directions: Vec<Coord> = [(0, -1), (1, 0), (0, 1), (-1, 0)].to_vec();
    let mut current_direction = 0;
    let map_size = (map[0].len() as i32, map.len() as i32);

    while is_in_bounds(current_position, map_size) {
        visited.insert(current_position);

        let mut new_position = add(&current_position, &directions[current_direction]);
        if !is_in_bounds(new_position, map_size) {
            break;
        }
        while map[new_position.1 as usize][new_position.0 as usize] == '#' {
            current_direction = (current_direction + 1) % 4;
            new_position = add(&current_position, &directions[current_direction]);
        }
        current_position = new_position;
    }

    println!("Part 1: {}", visited.len())
}

fn simulation_finishes(map: &Vec<Vec<char>>) -> bool {
    let mut visited: HashSet<(Coord, usize)> = HashSet::new();
    let mut current_position = get_current_position(&map);

    let directions: Vec<Coord> = [(0, -1), (1, 0), (0, 1), (-1, 0)].to_vec();
    let mut current_direction = 0;
    let map_size = (map[0].len() as i32, map.len() as i32);

    while is_in_bounds(current_position, map_size) {
        if let Some(_) = visited.get(&(current_position, current_direction)) {
            return false;
        }

        visited.insert((current_position, current_direction));

        let mut new_position = add(&current_position, &directions[current_direction]);
        if !is_in_bounds(new_position, map_size) {
            break;
        }
        while map[new_position.1 as usize][new_position.0 as usize] == '#' {
            current_direction = (current_direction + 1) % 4;
            new_position = add(&current_position, &directions[current_direction]);
        }
        current_position = new_position;
    }
    return true;
}

fn part_2() {
    let map = read_lines();
    let map_size = (map[0].len(), map.len());
    let mut possible_obstructions = 0;

    let bar = ProgressBar::new((map_size.0 * map_size.1) as u64);
    for i in 0..map_size.0 {
        for j in 0..map_size.1 {
            bar.inc(1);
            if map[i][j] == '^' || map[i][j] == '#' {
                continue;
            }
            let mut new_map = map.clone();
            new_map[i][j] = '#';
            if !simulation_finishes(&new_map) {
                possible_obstructions += 1;
            }
        }
    }
    bar.finish();

    println!("Part 2: {}", possible_obstructions)
}
fn main() {
    part_1();
    part_2();
}
