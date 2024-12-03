use regex::Regex;
use std::fs;

fn read_lines() -> Vec<String> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let mults: Vec<String> = re
        .find_iter(&contents)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();

    return mults;
}

fn part_1() {
    let mults = read_lines();
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    let mut multiplied: Vec<i32> = Vec::new();
    for mult in mults {
        let caps = re.captures(&mult).unwrap();
        let first = caps["first"].parse::<i32>().unwrap();
        let second = caps["second"].parse::<i32>().unwrap();
        multiplied.push(first * second);
    }
    let summed = multiplied.iter().sum::<i32>();
    println!("Part 1: {summed}");
}

fn read_lines_2() -> Vec<String> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let mults: Vec<String> = re
        .find_iter(&contents)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();

    return mults;
}

fn part_2() {
    let mults = read_lines_2();

    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let mut multiplied: Vec<i32> = Vec::new();
    let mut is_open = true;

    for mult in mults {
        match mult.chars().nth(0).unwrap() {
            'm' => {
                if is_open {
                    let caps = re.captures(&mult).unwrap();
                    let first = caps["first"].parse::<i32>().unwrap();
                    let second = caps["second"].parse::<i32>().unwrap();
                    multiplied.push(first * second);
                }
            }
            _ => {
                if mult == "do()" {
                    is_open = true;
                } else {
                    is_open = false;
                }
            }
        }
    }
    let summed = multiplied.iter().sum::<i32>();
    println!("Part 2: {summed}");
}

fn main() {
    part_1();
    part_2();
}
