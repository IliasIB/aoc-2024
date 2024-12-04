use std::fs;

fn read_lines() -> Vec<Vec<char>> {
    let file_path: &'static str = "input/puzzle.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rows = contents.lines().map(|x| x.chars().collect()).collect();

    return rows;
}

fn upper_left(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if (i - 3 >= 0) && (j - 3 >= 0) {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index - 3][col_index - 3] == 'S'
            && rows[row_index - 2][col_index - 2] == 'A'
            && rows[row_index - 1][col_index - 1] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn up(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if i - 3 >= 0 {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index - 3][col_index] == 'S'
            && rows[row_index - 2][col_index] == 'A'
            && rows[row_index - 1][col_index] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn upper_right(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if (i - 3 >= 0) && (j + 3 < rows.len() as i32) {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index - 3][col_index + 3] == 'S'
            && rows[row_index - 2][col_index + 2] == 'A'
            && rows[row_index - 1][col_index + 1] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn left(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if j - 3 >= 0 {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index][col_index - 3] == 'S'
            && rows[row_index][col_index - 2] == 'A'
            && rows[row_index][col_index - 1] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn right(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if j + 3 < rows[0].len() as i32 {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index][col_index + 3] == 'S'
            && rows[row_index][col_index + 2] == 'A'
            && rows[row_index][col_index + 1] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn lower_left(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if (i + 3 < rows.len() as i32) && (j - 3 >= 0) {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index + 3][col_index - 3] == 'S'
            && rows[row_index + 2][col_index - 2] == 'A'
            && rows[row_index + 1][col_index - 1] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn down(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if i + 3 < rows.len() as i32 {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index + 3][col_index] == 'S'
            && rows[row_index + 2][col_index] == 'A'
            && rows[row_index + 1][col_index] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn lower_right(rows: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if (i + 3 < rows.len() as i32) && (j + 3 < rows[0].len() as i32) {
        let row_index = i as usize;
        let col_index = j as usize;
        if rows[row_index + 3][col_index + 3] == 'S'
            && rows[row_index + 2][col_index + 2] == 'A'
            && rows[row_index + 1][col_index + 1] == 'M'
        {
            return true;
        }
        return false;
    }
    return false;
}

fn part_1() {
    let rows = read_lines();
    let column_size = rows.len();
    let row_size = rows[0].len();

    let mut xmas_count = 0;
    for i in 0..column_size {
        for j in 0..row_size {
            if rows[i][j] == 'X' {
                if upper_left(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if up(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if upper_right(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if left(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if right(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if lower_left(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if down(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };

                if lower_right(rows.clone(), i as i32, j as i32) {
                    xmas_count += 1
                };
            }
        }
    }
    println!("Part 1: {xmas_count}");
}

fn xmas_check(
    rows: Vec<Vec<char>>,
    upper_left: char,
    upper_right: char,
    lower_left: char,
    lower_right: char,
    i: usize,
    j: usize,
) -> bool {
    if rows[i][j] == upper_left
        && rows[i][j + 2] == upper_right
        && rows[i + 1][j + 1] == 'A'
        && rows[i + 2][j] == lower_left
        && rows[i + 2][j + 2] == lower_right
    {
        return true;
    }
    return false;
}

fn part_2() {
    let rows = read_lines();
    let mut xmas_count = 0;

    for i in 0..rows.len() - 2 {
        for j in 0..rows[0].len() - 2 {
            if xmas_check(rows.clone(), 'M', 'S', 'M', 'S', i, j) {
                xmas_count += 1;
            } else if xmas_check(rows.clone(), 'M', 'M', 'S', 'S', i, j) {
                xmas_count += 1;
            } else if xmas_check(rows.clone(), 'S', 'M', 'S', 'M', i, j) {
                xmas_count += 1;
            } else if xmas_check(rows.clone(), 'S', 'S', 'M', 'M', i, j) {
                xmas_count += 1;
            }
        }
    }
    println!("Part 2: {xmas_count}");
}

fn main() {
    part_1();
    part_2();
}
