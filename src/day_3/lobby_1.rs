use std::fs;

fn calculate_joltage(battery_bank: &str) -> u8 {
    let mut highest_battery_value: u8 = 0;
    let mut highest_battery_index: u8 = 0;
    for i in 0..battery_bank.len() - 1 {
        let battery = battery_bank.chars().nth(i).unwrap();
        let battery_value: u8 = battery.to_digit(10).unwrap() as u8;
        if battery_value > highest_battery_value {
            highest_battery_value = battery_value;
            highest_battery_index = i as u8;
        }
    }
    let mut second_highest_battery_value: u8 = 0;
    for i in (highest_battery_index + 1) as usize..battery_bank.len() {
        let battery = battery_bank.chars().nth(i).unwrap();
        let battery_value: u8 = battery.to_digit(10).unwrap() as u8;
        if battery_value > second_highest_battery_value {
            second_highest_battery_value = battery_value;
        }
    }
    highest_battery_value * 10 + second_highest_battery_value
}

pub fn solve() {
    let raw_file_content = fs::read_to_string("src/day_3/my_input.lst")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();

    let mut joltage_sum: u32 = 0;
    for battery_bank in file_content_as_lines {
        let joltage = calculate_joltage(&battery_bank);
        println!(
            "The calculated joltage for battery bank {} is: {}",
            battery_bank, joltage
        );
        joltage_sum += joltage as u32;
    }
    println!("The total joltage sum is: {}", joltage_sum);
}
