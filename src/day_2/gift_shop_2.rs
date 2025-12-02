use std::fs;

struct IdRange {
    start_id: u128,
    end_id: u128,
}

fn is_composed_of_repeated_sequence(full_string: &str, sequence: &str) -> bool {
    if !full_string.len().is_multiple_of(sequence.len()) || full_string.len() == sequence.len() {
        return false;
    }
    sequence.repeat(full_string.len() / sequence.len()) == full_string
}

fn is_id_invalid(id: &str) -> bool {
    let bigger_half = &id[..id.len() / 2 + 1];
    for i in 1..bigger_half.len() {
        let sequence = &bigger_half[0..i];
        if is_composed_of_repeated_sequence(id, sequence) {
            println!(
                "Found invalid ID: {} with repeating sequence {}",
                id, sequence
            );
            return true;
        }
    }
    false
}

pub fn solve() {
    let raw_file_content = fs::read_to_string("src/day_2/my_input.txt")
        .expect("Something went wrong reading the file");
    let split_file_content: Vec<&str> = raw_file_content.split(',').collect();
    let mut id_ranges: Vec<IdRange> = Vec::new();

    for entry in split_file_content {
        let entry_vec: Vec<&str> = entry.split('-').collect();
        let start_id: u128 = entry_vec[0].parse().unwrap();
        let end_id: u128 = entry_vec[1].parse().unwrap();
        id_ranges.push(IdRange { start_id, end_id });
    }

    let mut invalid_id_sum = 0;
    for range in id_ranges {
        for i in range.start_id..=range.end_id {
            if is_id_invalid(&i.to_string()) {
                invalid_id_sum += i;
            }
        }
    }
    println!("The sum of all invalid IDs is: {}", invalid_id_sum);
}
