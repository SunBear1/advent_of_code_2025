use std::fs;

struct IngredientIDRange {
    start: u64,
    end: u64,
}

fn load_input_from_file() -> (Vec<IngredientIDRange>, Vec<u64>) {
    let raw_file_content = fs::read_to_string("src/day_5/my_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();

    let mut ingredient_ranges: Vec<IngredientIDRange> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();

    let mut empty_line_found = false;
    for line in file_content_as_lines.iter() {
        if line.is_empty() {
            empty_line_found = true;
            continue;
        }
        if !empty_line_found {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ingredient_ranges.push(IngredientIDRange { start, end });
        } else {
            // After the empty line, we have the list of ingredient IDs
            let ingredient_id: u64 = line.parse().unwrap();
            ingredient_ids.push(ingredient_id);
        }
    }
    (ingredient_ranges, ingredient_ids)
}

fn is_ingredient_fresh(ingredient_id: u64, ingredient_ranges: &[IngredientIDRange]) -> bool {
    for range in ingredient_ranges.iter() {
        if ingredient_id >= range.start && ingredient_id <= range.end {
            return true;
        }
    }
    false
}

pub fn solve() {
    let (ingredient_ranges, list_of_ingredients) = load_input_from_file();
    let mut fresh_ingredients_counter = 0;
    for ingredient_id in list_of_ingredients.iter() {
        if is_ingredient_fresh(*ingredient_id, &ingredient_ranges) {
            fresh_ingredients_counter += 1;
        }
    }
    println!("Total fresh ingredients: {}", fresh_ingredients_counter);
}
