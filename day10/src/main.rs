use std::fs;

type Map = Vec<Vec<u32>>;
type Coord = (usize, usize);

fn read_lines() -> Map {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let map = contents
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Map>();

    return map;
}

fn get_trailheads(map: &Map) -> Vec<Coord> {
    let mut trailheads: Vec<Coord> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                trailheads.push((j, i));
            }
        }
    }
    return trailheads;
}

fn count_trails(map: &Map, start: &Coord, visited: &mut Vec<Coord>) -> u32 {
    let start_x = start.0 as i32;
    let start_y = start.1 as i32;
    let map_size = (map.len() as i32, map[0].len() as i32);

    if map[start.1][start.0] == 9 && !visited.contains(start) {
        visited.push(*start);
        return 1;
    }
    let mut trails: u32 = 0;
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for direction in directions {
        // println!("{:?} {:?}", start, direction);
        if start_x + direction.0 >= 0
            && start_x + direction.0 < map_size.0
            && start_y + direction.1 >= 0
            && start_y + direction.1 < map_size.1
            && map[(start_y + direction.1) as usize][(start_x + direction.0) as usize]
                == map[start.1][start.0] + 1
        {
            trails += count_trails(
                map,
                &(
                    (start_x + direction.0) as usize,
                    (start_y + direction.1) as usize,
                ),
                visited,
            );
        }
    }

    return trails;
}

fn part_1(map: &Map, trailheads: &Vec<Coord>) {
    let mut scores: Vec<u32> = Vec::new();
    for trailhead in trailheads {
        let mut visited: Vec<Coord> = Vec::new();
        scores.push(count_trails(map, &trailhead, &mut visited));
    }
    println!("Part 1: {}", scores.iter().sum::<u32>());
}

fn count_trails_2(map: &Map, start: &Coord) -> u32 {
    let start_x = start.0 as i32;
    let start_y = start.1 as i32;
    let map_size = (map.len() as i32, map[0].len() as i32);

    if map[start.1][start.0] == 9 {
        return 1;
    }
    let mut trails: u32 = 0;
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for direction in directions {
        // println!("{:?} {:?}", start, direction);
        if start_x + direction.0 >= 0
            && start_x + direction.0 < map_size.0
            && start_y + direction.1 >= 0
            && start_y + direction.1 < map_size.1
            && map[(start_y + direction.1) as usize][(start_x + direction.0) as usize]
                == map[start.1][start.0] + 1
        {
            trails += count_trails_2(
                map,
                &(
                    (start_x + direction.0) as usize,
                    (start_y + direction.1) as usize,
                ),
            );
        }
    }

    return trails;
}

fn part_2(map: &Map, trailheads: &Vec<Coord>) {
    let mut scores: Vec<u32> = Vec::new();
    for trailhead in trailheads {
        scores.push(count_trails_2(map, &trailhead));
    }
    println!("Part 2: {}", scores.iter().sum::<u32>());
}

fn main() {
    let map = read_lines();
    let trailheads = get_trailheads(&map);
    part_1(&map, &trailheads);
    part_2(&map, &trailheads);
}
