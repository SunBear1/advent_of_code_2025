use std::fs;

fn parse_cephalopod_input(file_content_as_lines: &[String]) -> Vec<Vec<String>> {
    let mut math_problems: Vec<Vec<String>> = Vec::new();
    let mut lines_split_by_whitespace: Vec<Vec<String>> = Vec::new();

    for line in file_content_as_lines {
        let line_split_by_whitespace: Vec<String> =
            line.split_whitespace().map(|s| s.to_string()).collect();
        lines_split_by_whitespace.push(line_split_by_whitespace);
    }

    let number_of_equations = lines_split_by_whitespace[0].len();
    let number_of_numbers_per_equation = lines_split_by_whitespace.len();

    for _ in 0..number_of_equations {
        math_problems.push(Vec::new());
    }

    for i in 0..number_of_equations {
        for row in lines_split_by_whitespace
            .iter()
            .take(number_of_numbers_per_equation)
        {
            let value = row[i].clone();
            math_problems[i].push(value);
        }
    }
    math_problems
}

fn load_input_from_file() -> Vec<Vec<String>> {
    let raw_file_content = fs::read_to_string("src/day_6/test_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();
    parse_cephalopod_input(&file_content_as_lines)
}

fn calculate_math_problem(math_problem: &[String]) -> u128 {
    let mut result: u128 = 1;
    let operation = &math_problem[math_problem.len() - 1];
    for problem in math_problem.iter().take(math_problem.len() - 1) {
        let value_as_int: u128 = problem.parse().unwrap();
        if operation == "+" {
            result += value_as_int;
        } else if operation == "*" {
            result *= value_as_int;
        }
    }
    if operation == "+" {
        result -= 1;
    }
    result
}

pub fn solve() {
    let math_problems = load_input_from_file();
    let mut result: u128 = 0;
    for math_problem in math_problems.iter() {
        result += calculate_math_problem(math_problem);
    }
    println!("The final result is: {}", result);
}
// 4412382293768
