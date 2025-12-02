use std::fs;

struct IdRange {
    start_id: u64,
    end_id: u64,
}

fn is_id_invalid(id: &str) -> bool {
    let id_length = id.len();
    let (left, right) = id.split_at(id_length / 2);
    left == right
}

pub fn solve() {
    let raw_file_content = fs::read_to_string("src/day_2/my_input.txt")
        .expect("Something went wrong reading the file");
    let split_file_content: Vec<&str> = raw_file_content.split(',').collect();
    let mut id_ranges: Vec<IdRange> = Vec::new();

    for entry in split_file_content {
        let entry_vec: Vec<&str> = entry.split('-').collect();
        let start_id: u64 = entry_vec[0].parse().unwrap();
        let end_id: u64 = entry_vec[1].parse().unwrap();
        id_ranges.push(IdRange { start_id, end_id });
    }

    let mut invalid_id_sum = 0;
    for range in id_ranges {
        for i in range.start_id..range.end_id {
            if is_id_invalid(&i.to_string()) {
                invalid_id_sum += i;
            }
        }
    }
    println!("The sum of all invalid IDs is: {}", invalid_id_sum);
}
