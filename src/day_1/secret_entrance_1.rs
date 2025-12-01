use std::fs;

struct SafeRotation {
    direction: char,
    step: i32,
}

const RIGHT_DIRECTION: char = 'R';
const LEFT_DIRECTION: char = 'L';
const SAFE_LOCK_SIZE: i32 = 100;

pub fn solve() {
    let raw_file_content = fs::read_to_string("src/day_1/my_input.lst")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();

    let mut safe_rotations: Vec<SafeRotation> = Vec::new();

    for code in file_content_as_lines {
        let direction = code.chars().next().unwrap();
        let step: i32 = code[1..].parse::<i32>().unwrap(); // we use modulo 100 to ensure we don't exceed the safe lock range
        safe_rotations.push(SafeRotation { direction, step });
    }

    let mut safe_lock_position = SAFE_LOCK_SIZE / 2;
    let mut password = 0;

    for rotation in safe_rotations {
        match rotation.direction {
            LEFT_DIRECTION => {
                if (safe_lock_position - rotation.step) == 0 {
                    safe_lock_position = 0;
                } else if (safe_lock_position - rotation.step) < 0 {
                    safe_lock_position = SAFE_LOCK_SIZE + (safe_lock_position - rotation.step); // if we go below 0, we wrap around from 100
                } else {
                    safe_lock_position -= rotation.step;
                }
            }
            RIGHT_DIRECTION => {
                if (safe_lock_position + rotation.step) > SAFE_LOCK_SIZE {
                    safe_lock_position = (safe_lock_position + rotation.step) - SAFE_LOCK_SIZE; // if we go above 99, we wrap around from 0
                } else if (safe_lock_position + rotation.step) == SAFE_LOCK_SIZE {
                    safe_lock_position = 0;
                } else {
                    safe_lock_position += rotation.step;
                }
            }
            _ => {}
        }
        if safe_lock_position == 0 {
            password += 1;
        }
    }
    println!("The safe lock password is: {}", password);
}
