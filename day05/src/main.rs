use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn read_lines() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rows: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut is_split_found = false;
    let mut counter = 0;
    while !is_split_found {
        let splitted: Vec<i32> = rows[counter]
            .split('|')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let first = splitted[0];
        let second = splitted[1];
        ordering_rules.entry(first).or_default().push(second);

        counter += 1;
        if rows[counter] == "" {
            is_split_found = true;
        }
    }

    let mut pages: Vec<Vec<i32>> = Vec::new();
    for i in counter + 1..rows.len() {
        pages.push(
            rows[i]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    return (ordering_rules, pages);
}

fn part_1() {
    let (ordering_rules, pages) = read_lines();
    let mut ordered_pages: Vec<Vec<i32>> = Vec::new();

    for page in pages {
        let mut is_ordered = true;
        for i in 0..page.len() - 1 {
            if let Some(rules) = ordering_rules.get(&page[i]) {
                for j in i + 1..page.len() {
                    if !rules.contains(&page[j]) {
                        is_ordered = false;
                        break;
                    }
                }
            } else {
                is_ordered = false;
                break;
            }
        }
        if is_ordered {
            ordered_pages.push(page);
        }
    }

    let summed: i32 = ordered_pages.iter().map(|x| x[x.len() / 2]).sum();
    println!("Part 1: {summed}");
}

fn part_2() {
    let (ordering_rules, pages) = read_lines();
    let mut ordered_pages: Vec<Vec<i32>> = Vec::new();
    let mut unordered_pages: Vec<Vec<i32>> = Vec::new();

    for page in pages {
        let mut is_ordered = true;
        for i in 0..page.len() - 1 {
            if let Some(rules) = ordering_rules.get(&page[i]) {
                for j in i + 1..page.len() {
                    if !rules.contains(&page[j]) {
                        is_ordered = false;
                        break;
                    }
                }
            } else {
                is_ordered = false;
                break;
            }
        }
        if is_ordered {
            ordered_pages.push(page);
        } else {
            unordered_pages.push(page);
        }
    }

    for page in &mut unordered_pages {
        page.sort_by(|number1: &i32, number2: &i32| {
            if let Some(rules) = ordering_rules.get(&number1) {
                if rules.contains(&number2) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Greater
            }
        });
    }

    let summed: i32 = unordered_pages.iter().map(|x| x[x.len() / 2]).sum();
    println!("Part 2: {summed}");
}

fn main() {
    part_1();
    part_2();
}
