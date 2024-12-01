use std::fs;

fn read_lines() -> (Vec<i32>, Vec<i32>, usize) {
    let file_path: &'static str = "input/distance.input";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vec_1: Vec<i32> = Vec::new();
    let mut vec_2: Vec<i32> = Vec::new();
    let mut size = 0;

    for line in contents.lines() {
        let (a, b) = line.split_once(char::is_whitespace).unwrap();
        vec_1.push(a.parse::<i32>().unwrap());
        vec_2.push(b.parse::<i32>().unwrap());
        size += 1;
    }

    return (vec_1, vec_2, size);
}

fn part_1() {
    let (mut vec_1, mut vec_2, size) = read_lines();
    vec_1.sort();
    vec_2.sort();

    let mut distances: Vec<i32> = Vec::new();
    for n in 0..size {
        distances.push((vec_1[n] - vec_2[n]).abs());
    }

    let sum = distances.iter().sum::<i32>();
    println!("Part 1: {sum}");
}

fn part_2() {
    let (vec_1, vec_2, size) = read_lines();

    let mut similarities: Vec<i32> = Vec::new();
    for n in 0..size {
        let similarity = vec_2.iter().filter(|&x| *x == vec_1[n]).count() as i32;
        similarities.push(similarity * vec_1[n]);
    }

    let sum = similarities.iter().sum::<i32>();
    println!("Part 2: {sum}");

}

fn main() {
    part_1();
    part_2();
}
