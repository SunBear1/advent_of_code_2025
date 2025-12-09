use std::fs;

fn load_input_from_file() -> Vec<Vec<char>> {
    let raw_file_content = fs::read_to_string("src/day_7/test_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();
    let diagram: Vec<Vec<char>> = file_content_as_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    diagram
}

fn advance_the_signal_in_diagram(
    diagram: &[Vec<char>],
    level: usize,
    signals_positions: Vec<usize>,
) -> (Vec<usize>, usize) {
    let mut new_signals_positions: Vec<usize> = Vec::new();
    let mut split_counter: usize = 0;

    for signal_position in signals_positions {
        if diagram[level][signal_position] == '.' {
            new_signals_positions.push(signal_position);
            println!("Signal at position {} continues straight.", signal_position);
        } else if diagram[level][signal_position] == '^' {
            new_signals_positions.push(signal_position - 1);
            new_signals_positions.push(signal_position + 1);
            println!(
                "Signal at position {} splits to positions {} and {}.",
                signal_position,
                signal_position - 1,
                signal_position + 1
            );
            split_counter += 1;
        }
    }
    new_signals_positions.sort();
    new_signals_positions.dedup();
    (new_signals_positions, split_counter)
}

fn locate_initial_signal_position(diagram: &[Vec<char>]) -> usize {
    for (index, cell) in diagram[0].iter().enumerate() {
        if *cell == 'S' {
            println!("Initial signal found at position {}.", index);
            return index;
        }
    }
    panic!("No initial signal found in the diagram.");
}

pub fn solve() {
    let tachyon_manifolds_diagram = load_input_from_file();
    let signals_positions: Vec<usize> =
        vec![locate_initial_signal_position(&tachyon_manifolds_diagram)];
    let mut split_counter_total: usize = 0;

    let mut new_signals: Vec<usize> = signals_positions.clone();
    for i in 1..tachyon_manifolds_diagram.len() {
        let (new_signals_tmp, split_counter) =
            advance_the_signal_in_diagram(&tachyon_manifolds_diagram, i, new_signals);
        new_signals = new_signals_tmp;
        println!("After level {}, signal positions: {:?}", i + 1, new_signals);
        split_counter_total += split_counter;
    }
    println!("Total number of splits: {}", split_counter_total);
}
