use std::{fs, time::Instant};

type Equation = (usize, Vec<usize>);

fn read_lines() -> Vec<Equation> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents
        .lines()
        .map(|x| x.split(": ").map(|x| x.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    let mut equations: Vec<Equation> = Vec::new();
    for line in lines {
        equations.push((
            line[0].parse::<usize>().unwrap(),
            line[1]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        ));
    }

    return equations;
}

fn is_result_of(result: &usize, numbers: &Vec<usize>, iterator: usize, collector: usize) -> bool {
    if iterator == numbers.len() - 2 {
        if collector + numbers[iterator + 1] == *result {
            return true;
        } else if collector * numbers[iterator + 1] == *result {
            return true;
        } else {
            return false;
        }
    } else if is_result_of(
        result,
        numbers,
        iterator + 1,
        collector + numbers[iterator + 1],
    ) {
        return true;
    } else if is_result_of(
        result,
        numbers,
        iterator + 1,
        collector * numbers[iterator + 1],
    ) {
        return true;
    } else {
        return false;
    }
}

fn part_1(equations: &Vec<Equation>) {
    let start = Instant::now();
    let solvable_equations: usize = equations
        .iter()
        .filter(|(result, numbers)| is_result_of(result, numbers, 0, numbers[0]))
        .map(|(result, _)| *result)
        .sum();
    let duration = start.elapsed();

    println!("Part 1: {solvable_equations}, solved in {:?}", duration);
}

fn concat(number1: &usize, number2: &usize) -> usize {
    return (number1.to_string() + &number2.to_string())
        .parse::<usize>()
        .unwrap();
}

fn is_result_of2(result: &usize, numbers: &Vec<usize>, iterator: usize, collector: usize) -> bool {
    if iterator == numbers.len() - 2 {
        if collector + numbers[iterator + 1] == *result {
            return true;
        } else if collector * numbers[iterator + 1] == *result {
            return true;
        } else if concat(&collector, &numbers[iterator + 1]) == *result {
            return true;
        } else {
            return false;
        }
    } else if is_result_of2(
        result,
        numbers,
        iterator + 1,
        collector + numbers[iterator + 1],
    ) {
        return true;
    } else if is_result_of2(
        result,
        numbers,
        iterator + 1,
        collector * numbers[iterator + 1],
    ) {
        return true;
    } else if is_result_of2(
        result,
        numbers,
        iterator + 1,
        concat(&collector, &numbers[iterator + 1]),
    ) {
        return true;
    } else {
        return false;
    }
}

fn part_2(equations: &Vec<Equation>) {
    let start = Instant::now();
    let solvable_equations: usize = equations
        .iter()
        .filter(|(result, numbers)| is_result_of2(result, numbers, 0, numbers[0]))
        .map(|(result, _)| *result)
        .sum();
    let duration = start.elapsed();

    println!("Part 2: {solvable_equations}, solved in {:?}", duration);
}

fn main() {
    let equations = read_lines();
    part_1(&equations);
    part_2(&equations);
}
