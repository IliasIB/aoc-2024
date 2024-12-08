use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Coord = (i32, i32);

fn read_lines() -> Vec<Vec<char>> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let map: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();

    return map;
}

fn get_antinodes_for_antennas(map_size: Coord, antennas: Vec<Coord>) -> Vec<Coord> {
    let mut antinodes: Vec<Coord> = Vec::new();
    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let slope_x = antennas[j].1 - antennas[i].1;
            let slope_y = antennas[j].0 - antennas[i].0;

            let antinode_1 = (antennas[i].0 - slope_y, antennas[i].1 - slope_x);
            let antinode_2 = (antennas[j].0 + slope_y, antennas[j].1 + slope_x);

            if antinode_1.0 >= 0
                && antinode_1.0 < map_size.0
                && antinode_1.1 >= 0
                && antinode_1.1 < map_size.1
            {
                antinodes.push(antinode_1);
            }
            if antinode_2.0 >= 0
                && antinode_2.0 < map_size.0
                && antinode_2.1 >= 0
                && antinode_2.1 < map_size.1
            {
                antinodes.push(antinode_2);
            }
        }
    }
    return antinodes;
}

fn part_1(map: &Vec<Vec<char>>) {
    let map_size = (map.len() as i32, map[0].len() as i32);
    let mut antenna_lists: HashMap<char, Vec<Coord>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let element = map[i][j];
            if map[i][j] != '.' {
                antenna_lists
                    .entry(element)
                    .or_default()
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for antenna_list in antenna_lists {
        for antinode in get_antinodes_for_antennas(map_size, antenna_list.1) {
            antinodes.insert(antinode);
        }
    }

    println!("Part 1: {}", antinodes.len());
}

fn add_antinodes(
    start: Coord,
    map_size: Coord,
    slope_x: i32,
    slope_y: i32,
    antinodes: &mut Vec<Coord>,
) {
    let mut current_pos = (start.0 + slope_y, start.1 + slope_x);
    while current_pos.0 >= 0
        && current_pos.0 < map_size.0
        && current_pos.1 >= 0
        && current_pos.1 < map_size.1
    {
        antinodes.push(current_pos);
        current_pos = (current_pos.0 + slope_y, current_pos.1 + slope_x);
    }
}

fn get_antinodes_for_antennas_2(map_size: Coord, antennas: Vec<Coord>) -> Vec<Coord> {
    let mut antinodes: Vec<Coord> = Vec::new();
    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let slope_x = antennas[j].1 - antennas[i].1;
            let slope_y = antennas[j].0 - antennas[i].0;

            add_antinodes(antennas[j], map_size, slope_x, slope_y, &mut antinodes);
            add_antinodes(antennas[i], map_size, -slope_x, -slope_y, &mut antinodes);

            antinodes.push(antennas[j]);
            antinodes.push(antennas[i]);
        }
    }
    return antinodes;
}

fn part_2(map: &Vec<Vec<char>>) {
    let map_size = (map.len() as i32, map[0].len() as i32);
    let mut antenna_lists: HashMap<char, Vec<Coord>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let element = map[i][j];
            if map[i][j] != '.' {
                antenna_lists
                    .entry(element)
                    .or_default()
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for antenna_list in antenna_lists {
        for antinode in get_antinodes_for_antennas_2(map_size, antenna_list.1) {
            antinodes.insert(antinode);
        }
    }

    println!("Part 2: {}", antinodes.len());
}

fn main() {
    let map = read_lines();
    part_1(&map);
    part_2(&map);
}
