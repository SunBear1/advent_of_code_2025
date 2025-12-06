use std::fs;

type CephalopodMathProblem = (Vec<String>, char);

fn replace_whitespace_columns_with_markers(file_content_as_lines: &[String]) -> Vec<String> {
    let mut owned_file_content_as_lines = file_content_as_lines.to_owned();
    let mut whitespace_columns: Vec<usize> = Vec::new();
    let number_of_columns = file_content_as_lines[0].len();

    for col in 0..number_of_columns {
        let mut is_whitespace_column = true;
        for line in file_content_as_lines {
            if let Some(c) = line.chars().nth(col)
                && !c.is_whitespace()
            {
                is_whitespace_column = false;
                break;
            }
        }
        if is_whitespace_column {
            whitespace_columns.push(col);
        }
    }

    for &col in whitespace_columns.iter().rev() {
        for line in owned_file_content_as_lines.iter_mut() {
            line.replace_range(col..col + 1, "M");
        }
    }
    owned_file_content_as_lines
}

fn parse_cephalopod_input(file_content_as_lines: &[String]) -> Vec<CephalopodMathProblem> {
    let mut math_problems: Vec<Vec<String>> = Vec::new();
    let mut lines_split_by_markers: Vec<Vec<String>> = Vec::new();

    let marked_file_content_as_lines =
        replace_whitespace_columns_with_markers(file_content_as_lines);

    for line in marked_file_content_as_lines {
        let line_split_by_markers: Vec<String> = line.split('M').map(|s| s.to_string()).collect();
        lines_split_by_markers.push(line_split_by_markers);
    }

    let number_of_equations = lines_split_by_markers[0].len();
    let number_of_numbers_per_equation = lines_split_by_markers.len();

    for _ in 0..number_of_equations {
        math_problems.push(Vec::new());
    }

    for i in 0..number_of_equations {
        for row in lines_split_by_markers
            .iter()
            .take(number_of_numbers_per_equation)
        {
            let value = row[i].clone();
            math_problems[i].push(value);
        }
    }

    let mut parsed_math_problems: Vec<CephalopodMathProblem> = Vec::new();
    for problem in math_problems.iter() {
        let operation = problem[problem.len() - 1].chars().next().unwrap();
        let equation = problem[..problem.len() - 1].to_vec();
        parsed_math_problems.push((equation, operation));
    }
    parsed_math_problems
}

fn build_equations_from_matrix(equation_matrix: &[Vec<char>]) -> Vec<usize> {
    let mut equations: Vec<String> = Vec::new();
    let number_of_rows = equation_matrix.len();

    for _ in 0..number_of_rows {
        equations.push(String::new());
    }

    for row in 0..number_of_rows {
        for digit in &equation_matrix[row] {
            if *digit != ' ' {
                equations[row].push(*digit);
            }
        }
    }

    let mut numeric_equations: Vec<usize> = Vec::new();
    for equation in equations.iter() {
        let equation_as_int: usize = equation.parse().unwrap();
        numeric_equations.push(equation_as_int);
    }
    numeric_equations
}

fn load_input_from_file() -> Vec<(Vec<usize>, char)> {
    let raw_file_content = fs::read_to_string("src/day_6/test_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();
    let mut transformed_equations: Vec<(Vec<usize>, char)> = Vec::new();
    for parsed_input in parse_cephalopod_input(&file_content_as_lines).iter() {
        let equation_matrix = vector_of_strings_to_char_matrix(&parsed_input.0);
        let equations = build_equations_from_matrix(&equation_matrix);
        transformed_equations.push((equations, parsed_input.1));
    }
    transformed_equations
}

fn vector_of_strings_to_char_matrix(equation: &[String]) -> Vec<Vec<char>> {
    let rows = equation.iter().map(|s| s.len()).max().unwrap_or(0);
    let cols = equation.len();
    let mut equation_matrix: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    for i in 0..cols {
        let number = &equation[i];
        for (j, digit) in number.chars().enumerate() {
            equation_matrix[rows - j - 1][i] = digit;
        }
    }
    equation_matrix
}

fn calculate_math_problem(math_problem: (Vec<usize>, char)) -> u128 {
    let mut result: u128 = 1;
    let operation = math_problem.1;
    let math_problem = math_problem.0;

    for problem in math_problem.iter() {
        let value_as_int: u128 = *problem as u128;
        if operation == '+' {
            result += value_as_int;
        } else if operation == '*' {
            result *= value_as_int;
        }
    }
    if operation == '+' {
        result -= 1;
    }
    result
}

pub fn solve() {
    let math_problems = load_input_from_file();
    let mut result: u128 = 0;
    for math_problem in math_problems.iter() {
        result += calculate_math_problem(math_problem.clone());
    }
    println!("The final result is: {}", result);
}
// 7858808482092
