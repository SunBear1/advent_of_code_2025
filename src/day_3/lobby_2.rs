use std::fs;

const BATTERY_JOLTAGE_SIZE: usize = 12;

fn calculate_highest_battery_value(
    battery_bank: &str,
    start_index: u8,
    joltage_number_size: usize,
) -> (u8, u8) {
    let mut highest_battery_value: u8 = 0;
    let mut highest_battery_index: u8 = 0;
    for i in start_index as usize..battery_bank.len() - joltage_number_size {
        let battery = battery_bank.chars().nth(i).unwrap();
        let battery_value: u8 = battery.to_digit(10).unwrap() as u8;
        if battery_value > highest_battery_value {
            highest_battery_value = battery_value;
            highest_battery_index = i as u8;
        }
    }
    (highest_battery_value, highest_battery_index)
}

fn calculate_joltage(battery_bank: &str) -> u64 {
    let mut total_joltage: u64 = 0;
    let mut start_index: u8 = 0;
    let mut joltage_number_size: usize = BATTERY_JOLTAGE_SIZE;
    for i in 1..=BATTERY_JOLTAGE_SIZE {
        joltage_number_size -= 1;
        let (highest_battery_value, new_start_index) =
            calculate_highest_battery_value(battery_bank, start_index, joltage_number_size);
        total_joltage +=
            (highest_battery_value as u64) * 10u64.pow((BATTERY_JOLTAGE_SIZE - i) as u32);
        start_index = new_start_index + 1;
    }
    total_joltage
}

pub fn solve() {
    let raw_file_content = fs::read_to_string("src/day_3/my_input.lst")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();

    let mut joltage_sum: u128 = 0;
    for battery_bank in file_content_as_lines {
        let joltage = calculate_joltage(&battery_bank);
        println!(
            "The calculated joltage for battery bank {} is: {}",
            battery_bank, joltage
        );
        joltage_sum += joltage as u128;
    }
    println!("The total joltage sum is: {}", joltage_sum);
}
//172162399742349
