mod day_1;

const DAY: u8 = 1;
const PART: u8 = 2;

fn main() {
    match (DAY, PART) {
        (1, 1) => day_1::secret_entrance_1::solve(),
        (1, 2) => day_1::secret_entrance_2::solve(),
        _ => println!("Day {} Part {} not implemented", DAY, PART),
    }
}
