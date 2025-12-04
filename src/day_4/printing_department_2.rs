use std::fs;

fn get_value_from_department_grid_position(
    department_plan: &[Vec<char>],
    row: isize,
    col: isize,
) -> Option<char> {
    let number_of_rows = department_plan.len() as isize;
    let number_of_columns = department_plan[0].len() as isize;

    if row < 0 || row >= number_of_rows || col < 0 || col >= number_of_columns {
        return None;
    }
    Some(department_plan[row as usize][col as usize])
}

fn count_adjacent_rolls_of_paper(department_plan: &[Vec<char>], row: isize, col: isize) -> u8 {
    let mut count: u8 = 0;
    let adjacent_positions: [Option<char>; 8] = [
        get_value_from_department_grid_position(department_plan, row - 1, col),
        get_value_from_department_grid_position(department_plan, row + 1, col),
        get_value_from_department_grid_position(department_plan, row, col - 1),
        get_value_from_department_grid_position(department_plan, row, col + 1),
        get_value_from_department_grid_position(department_plan, row - 1, col - 1),
        get_value_from_department_grid_position(department_plan, row - 1, col + 1),
        get_value_from_department_grid_position(department_plan, row + 1, col - 1),
        get_value_from_department_grid_position(department_plan, row + 1, col + 1),
    ];
    for position in adjacent_positions.iter() {
        if position == &Some('@') {
            count += 1;
        }
    }
    count
}

pub fn find_forklift_accessible_spots(department_plan: &[Vec<char>]) -> Vec<(usize, usize)> {
    let number_of_rows = department_plan.len();
    let number_of_columns = department_plan[0].len();
    let mut rolls_of_paper_to_remove: Vec<(usize, usize)> = Vec::new();

    for i in 0..number_of_rows {
        for j in 0..number_of_columns {
            if department_plan[i][j] == '@'
                && 4 > count_adjacent_rolls_of_paper(department_plan, i as isize, j as isize)
            {
                rolls_of_paper_to_remove.push((i, j));
                println!("Found accessible spot at row {}, column {}", i, j);
            }
        }
    }
    rolls_of_paper_to_remove
}

pub fn remove_rolls_of_paper(
    department_plan: &mut [Vec<char>],
    spots_to_remove: &[(usize, usize)],
) {
    for spot in spots_to_remove.iter() {
        let (row, col) = *spot;
        department_plan[row][col] = '.';
    }
}

fn load_input_from_file() -> Vec<Vec<char>> {
    let raw_file_content = fs::read_to_string("src/day_4/my_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();

    let number_of_rows = file_content_as_lines.len();
    let number_of_columns = file_content_as_lines[0].len();
    let mut department_plan: Vec<Vec<char>> = vec![vec!['N'; number_of_columns]; number_of_rows];

    for (i, row) in file_content_as_lines.iter().enumerate() {
        for (j, symbol) in row.chars().enumerate() {
            department_plan[i][j] = symbol;
        }
    }
    department_plan
}

pub fn solve() {
    let mut department_plan = load_input_from_file();

    let mut removed_rolls_of_paper = 0;
    loop {
        let spots_to_remove = find_forklift_accessible_spots(&department_plan);
        if spots_to_remove.is_empty() {
            break;
        }
        removed_rolls_of_paper += spots_to_remove.len();
        remove_rolls_of_paper(&mut department_plan, &spots_to_remove);
    }

    println!(
        "The number of forklift accessible spots is: {}",
        removed_rolls_of_paper
    );
}
