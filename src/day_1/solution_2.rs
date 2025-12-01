pub fn run() {
    let puzzle_input: Vec<&str> = include_str!("./example.txt").split("\n").collect();
    let mut rotations: Vec<(char, i32)> = vec![];
    let mut result: i32 = 0;
    for rotation in puzzle_input.iter() {
        let direction = (match rotation.get(0..1) {
            Some(value) => value,
            None => "",
        })
        .chars()
        .collect::<Vec<char>>()[0];
        let size_of_rotation: i32 = (match rotation.get(1..) {
            Some(value) => value,
            None => "0",
        })
        .parse()
        .unwrap();
        rotations.push((direction, size_of_rotation));
    }

    let mut current_dial: i32 = 50;
    for (direction, size_of_spin) in rotations.iter() {
        let initital_dial = current_dial;
        match direction {
            'L' => current_dial -= size_of_spin,
            'R' => current_dial += size_of_spin,
            _ => {}
        }

        if current_dial > 100 {
            result += current_dial / 100;
            if current_dial % 100 == 0 {
                result -= 1;
            }
        } else if current_dial < 0 {
            result += if current_dial % 100 != 0 { 1 } else { 0 };
            result += (current_dial / 100).abs();
            if initital_dial == 0 {
                result -= 1;
            }
        }

        current_dial %= 100;
        if current_dial < 0 {
            current_dial = 100 + current_dial;
        }

        if current_dial == 0 {
            result += 1;
        }
    }

    println!("Day 1 Part 2 res is: {}", result);
}
