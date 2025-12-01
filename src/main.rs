mod day_1;
mod demo;

const DAY: u8 = 1;
const PART: u8 = 2;

fn main() {
    match (DAY, PART) {
        (0, 1) => demo::solution_1::run(),
        (0, 2) => demo::solution_2::run(),
        (1, 1) => day_1::solution_1::run(),
        (1, 2) => day_1::solution_2::run(),
        _ => println!("Day {} Part {} not implemented", DAY, PART),
    }
}
