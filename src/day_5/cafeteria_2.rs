use std::cmp::{max, min};
use std::fs;

type IngredientIDRange = (u128, u128);

fn load_input_from_file() -> Vec<IngredientIDRange> {
    let raw_file_content = fs::read_to_string("src/day_5/my_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();

    let mut ingredient_ranges: Vec<IngredientIDRange> = Vec::new();

    for line in file_content_as_lines.iter() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('-').collect();
        let start: u128 = parts[0].parse().unwrap();
        let end: u128 = parts[1].parse().unwrap();
        ingredient_ranges.push((start, end));
    }
    ingredient_ranges
}

fn is_range_within(range1: IngredientIDRange, range2: IngredientIDRange) -> bool {
    let range1_start = range1.0;
    let range1_end = range1.1;
    let range2_start = range2.0;
    let range2_end = range2.1;

    if range1_start < range2_start && range1_end > range2_end {
        return true; // range2 is within range1
    }
    false
}

fn are_ranges_overlapping(range1: IngredientIDRange, range2: IngredientIDRange) -> bool {
    let range1_start = range1.0;
    let range1_end = range1.1;
    let range2_start = range2.0;
    let range2_end = range2.1;

    range2_start <= range1_end && range2_end >= range1_start
}

fn filer_out_redundant_ranges(
    ingredient_ranges: &[IngredientIDRange],
) -> (Vec<IngredientIDRange>, Vec<IngredientIDRange>) {
    let mut ranged_to_add: Vec<IngredientIDRange> = Vec::new();
    let mut ranges_to_remove: Vec<IngredientIDRange> = Vec::new();
    for i in 0..ingredient_ranges.len() {
        for j in 0..ingredient_ranges.len() {
            if i == j {
                continue;
            }
            if is_range_within(ingredient_ranges[i], ingredient_ranges[j]) {
                println!(
                    "The range {:?} is within the range {:?}",
                    ingredient_ranges[j], ingredient_ranges[i]
                );
                ranges_to_remove.push((ingredient_ranges[j].0, ingredient_ranges[j].1));
            } else if are_ranges_overlapping(ingredient_ranges[i], ingredient_ranges[j]) {
                let new_start = min(ingredient_ranges[i].0, ingredient_ranges[j].0);
                let new_end = max(ingredient_ranges[i].1, ingredient_ranges[j].1);

                if ranged_to_add.contains(&(new_start, new_end)) {
                    continue;
                }

                println!(
                    "Ranges {:?} and {:?} are overlapping",
                    ingredient_ranges[i], ingredient_ranges[j]
                );
                println!("Creating new range from {} to {}", new_start, new_end);
                ranged_to_add.push((new_start, new_end));
                ranges_to_remove.push((ingredient_ranges[i].0, ingredient_ranges[i].1));
                ranges_to_remove.push((ingredient_ranges[j].0, ingredient_ranges[j].1));
            }
        }
    }
    ranged_to_add.sort();
    ranges_to_remove.sort();
    ranged_to_add.dedup();
    ranges_to_remove.dedup();

    println!("Ranges to remove: {:?}", ranges_to_remove);
    println!("Ranges to add: {:?}", ranged_to_add);
    (ranged_to_add, ranges_to_remove)
}

fn calculate_sum_of_ranges(ingredient_ranges: &[IngredientIDRange]) -> u128 {
    let mut total_sum: u128 = 0;
    for range in ingredient_ranges.iter() {
        total_sum += range.1 - range.0 + 1;
    }
    total_sum
}

pub fn solve() {
    let ingredient_ranges = load_input_from_file();

    let mut filtered_ranges = ingredient_ranges.clone();

    loop {
        filtered_ranges.sort();
        filtered_ranges.dedup();
        let (ranges_to_add, ranges_to_remove) = filer_out_redundant_ranges(&filtered_ranges);
        if ranges_to_add.is_empty() && ranges_to_remove.is_empty() {
            break;
        }

        filtered_ranges.retain(|range| !ranges_to_remove.contains(range));

        for range in ranges_to_add.iter() {
            if !filtered_ranges.contains(range) {
                filtered_ranges.push(*range);
            }
        }
        println!(
            "Updated filtered ranges after this iteration: {:?}",
            filtered_ranges
        );
    }
    let total_ingredients = calculate_sum_of_ranges(&filtered_ranges);
    filtered_ranges.sort();
    println!("Filtered ranges: {:?}", filtered_ranges);
    println!("Total fresh ingredients: {}", total_ingredients);
}
