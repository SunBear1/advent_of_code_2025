mod day_1;
mod day_2;
mod day_3;
mod day_4;

const DAY: u8 = 4;
const PART: u8 = 1;

fn main() {
    match (DAY, PART) {
        (1, 1) => day_1::secret_entrance_1::solve(),
        (1, 2) => day_1::secret_entrance_2::solve(),
        (2, 1) => day_2::gift_shop_1::solve(),
        (2, 2) => day_2::gift_shop_2::solve(),
        (3, 1) => day_3::lobby_1::solve(),
        (3, 2) => day_3::lobby_2::solve(),
        (4, 1) => day_4::printing_department_1::solve(),
        (4, 2) => day_4::printing_department_2::solve(),
        _ => println!("Day {} Part {} not implemented", DAY, PART),
    }
}
