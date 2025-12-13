use std::cmp::Reverse;
use std::fs;
use std::iter::Iterator;

type JunctionBoxLocation = (u32, u32, u32);

const CONNECTION_LIMIT: usize = 1000;

fn calculate_distance_between_junction_boxes_squared(
    box1: JunctionBoxLocation,
    box2: JunctionBoxLocation,
) -> u64 {
    let dx = box2.0 as i64 - box1.0 as i64;
    let dy = box2.1 as i64 - box1.1 as i64;
    let dz = box2.2 as i64 - box1.2 as i64;

    (dx * dx + dy * dy + dz * dz) as u64
}

fn load_input_from_file() -> Vec<JunctionBoxLocation> {
    let raw_file_content = fs::read_to_string("src/day_8/my_input.txt")
        .expect("Something went wrong reading the file");
    let file_content_as_lines: Vec<String> =
        raw_file_content.lines().map(|s| s.to_string()).collect();
    let mut boxes: Vec<JunctionBoxLocation> = Vec::new();
    for line in &file_content_as_lines {
        let parts: Vec<&str> = line.split(',').collect();
        let x: u32 = parts[0].parse().unwrap();
        let y: u32 = parts[1].parse().unwrap();
        let z: u32 = parts[2].parse().unwrap();
        boxes.push((x, y, z));
    }
    boxes
}

fn merge_two_circuits(
    circuits: Vec<Vec<JunctionBoxLocation>>,
    mut circuit1: Vec<JunctionBoxLocation>,
    mut circuit2: Vec<JunctionBoxLocation>,
    directly_connected_boxes: Vec<(JunctionBoxLocation, JunctionBoxLocation)>,
) -> Vec<Vec<JunctionBoxLocation>> {
    let mut circuits = circuits;

    if circuit1 == circuit2 {
        return circuits;
    }

    for circuit in &circuits {
        for junction_box in circuit {
            if circuit1[0] == *junction_box {
                circuit1 = circuit.clone();
            }
            if circuit2[0] == *junction_box {
                circuit2 = circuit.clone();
            }
        }
    }

    for (box1, box2) in &directly_connected_boxes {
        if (circuit1.contains(box1) && circuit2.contains(box2))
            || (circuit1.contains(box2) && circuit2.contains(box1))
        {
            println!(
                "Circuits already directly connected: {:?} and {:?}",
                circuit1, circuit2
            );
            return circuits;
        }
    }

    println!("Merging circuits: {:?} and {:?}", circuit1, circuit2);

    let index = circuits.iter().position(|x| *x == circuit1).unwrap();
    circuit1.append(&mut circuit2.clone());
    circuits[index] = circuit1;

    let index = circuits.iter().position(|x| *x == circuit2).unwrap();
    circuits.remove(index);
    circuits
}

pub fn solve() {
    let boxes = load_input_from_file();
    let boxes_quantity: usize = boxes.len();

    let mut edges: Vec<(u64, JunctionBoxLocation, JunctionBoxLocation)> =
        Vec::with_capacity(boxes_quantity * (boxes_quantity - 1) / 2);
    for i in 0..boxes_quantity {
        for j in (i + 1)..boxes_quantity {
            let dist = calculate_distance_between_junction_boxes_squared(boxes[i], boxes[j]);
            edges.push((dist, boxes[i], boxes[j]));
        }
    }
    edges.sort_unstable_by_key(|k| k.0);

    let mut circuits: Vec<Vec<JunctionBoxLocation>> = Vec::new();
    for b in &boxes {
        circuits.push(vec![*b]);
    }

    let mut directly_connected_boxes: Vec<(JunctionBoxLocation, JunctionBoxLocation)> = Vec::new();
    for (i, edge) in edges.iter().enumerate().take(CONNECTION_LIMIT) {
        let current_circuits = merge_two_circuits(
            circuits,
            vec![edge.1],
            vec![edge.2],
            directly_connected_boxes.clone(),
        );
        directly_connected_boxes.push((edge.1, edge.2));
        circuits = current_circuits;
        println!("Connections made: {}", i + 1);
    }
    circuits.sort_unstable_by_key(|k| Reverse(k.len()));
    for (i, circuit) in circuits.iter().enumerate() {
        println!("Circuit {}: {:?}", i + 1, circuit.len(),);
    }
    let mut result = 1;
    for circuit in circuits.iter().take(3) {
        result *= circuit.len();
    }
    println!("Result: {}", result);
}
