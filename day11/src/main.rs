use std::{collections::HashMap, fs};

fn read_lines() -> Vec<usize> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let numbers = contents
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return numbers;
}

fn part1(numbers: Vec<usize>) {
    let mut current_numbers = numbers;
    let mut count = current_numbers.len();
    for _ in 0..25 {
        let mut numbers2: Vec<usize> = Vec::new();

        for number in &current_numbers {
            if number == &0 {
                numbers2.push(1);
            } else if number.to_string().len() % 2 == 0 {
                let mut number_string = number.to_string();
                let number_length = number_string.len();
                let number_string2 = number_string.split_off(number_length / 2);
                numbers2.push(number_string.parse::<usize>().unwrap());
                numbers2.push(number_string2.parse::<usize>().unwrap());
                count += 1;
            } else {
                numbers2.push(number * 2024);
            }
        }
        current_numbers = numbers2;
    }
    println!("Part 1: {}", count);
}

fn get_stone_count(
    number: usize,
    cache: &mut HashMap<(usize, usize), usize>,
    blinks: usize,
) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(value) = cache.get(&(number, blinks)) {
        return *value;
    }
    let count: usize;
    if number == 0 {
        count = get_stone_count(1, cache, blinks - 1);
    } else if number.to_string().len() % 2 == 0 {
        let mut number_string = number.to_string();
        let number_length = number_string.len();
        let number_string2 = number_string.split_off(number_length / 2);
        let number1 = number_string.parse::<usize>().unwrap();
        let number2 = number_string2.parse::<usize>().unwrap();
        count = get_stone_count(number1, cache, blinks - 1)
            + get_stone_count(number2, cache, blinks - 1);
    } else {
        count = get_stone_count(number * 2024, cache, blinks - 1);
    }
    cache.insert((number, blinks), count);
    return count;
}

fn part2(numbers: Vec<usize>) {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let mut count = 0;
    for number in numbers {
        count += get_stone_count(number, &mut cache, 75);
    }

    println!("Part 2: {}", count);
}

fn main() {
    let numbers = read_lines();
    part1(numbers.clone());
    part2(numbers);
}
