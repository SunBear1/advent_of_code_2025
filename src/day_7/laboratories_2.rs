use std::fs;

fn load_input_from_file() -> Vec<Vec<String>> {
    let raw_file_content = fs::read_to_string("src/day_7/my_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();
    let diagram: Vec<Vec<String>> = file_content_as_lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    diagram
}

fn locate_initial_signal_position(diagram: &[Vec<String>]) -> usize {
    for (index, cell) in diagram[0].iter().enumerate() {
        if *cell == "S" {
            println!("Initial signal found at position {}.", index);
            return index;
        }
    }
    panic!("No initial signal found in the diagram.");
}

fn evaluate_possible_journeys_in_a_row(caret_row: &[String], number_row: &[String]) -> Vec<String> {
    let mut result_row: Vec<usize> = vec![0; number_row.len()];

    for i in 0..number_row.len() {
        if number_row[i] != "." {
            let number_row_as_number: usize = number_row[i].parse().unwrap();
            if caret_row[i] == "^" {
                result_row[i - 1] += number_row_as_number;
                result_row[i + 1] += number_row_as_number;
            } else {
                result_row[i] += number_row_as_number;
            }
        }
    }
    result_row
        .iter()
        .map(|num| {
            if *num == 0 {
                ".".to_string()
            } else {
                num.to_string()
            }
        })
        .collect()
}

fn mark_possible_journeys_on_diagram(
    diagram: &mut [Vec<String>],
    signal_start_position: usize,
) -> Vec<Vec<String>> {
    diagram[1][signal_start_position] = "1".to_string();

    for i in (3..diagram.len()).step_by(2) {
        let next_number_row = evaluate_possible_journeys_in_a_row(&diagram[i - 1], &diagram[i - 2]);
        diagram[i] = next_number_row;
    }
    diagram.to_vec()
}

fn sum_possible_journeys_in_a_row(number_row: &[String]) -> usize {
    let mut sum: usize = 0;
    for cell in number_row {
        if *cell != "." {
            let cell_as_number: usize = cell.parse().unwrap();
            sum += cell_as_number;
        }
    }
    sum
}

pub fn solve() {
    let tachyon_manifolds_diagram = load_input_from_file();
    let signal_start_position: usize = locate_initial_signal_position(&tachyon_manifolds_diagram);
    let journeys_marked = mark_possible_journeys_on_diagram(
        &mut tachyon_manifolds_diagram.clone(),
        signal_start_position,
    );
    let result = sum_possible_journeys_in_a_row(journeys_marked.last().unwrap());
    println!("Total possible journeys: {}", result);
}
