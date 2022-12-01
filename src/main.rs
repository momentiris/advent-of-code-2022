use std::fs::{self};

fn day_01() {
    let raw_input = fs::read_to_string("src/day-01/input.txt").expect("Failed to read input");

    let parsed_input: Vec<Vec<i32>> = raw_input
        .split("\n\n")
        .map(|x: &str| x.split("\n").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let mut result: Vec<i32> = parsed_input
        .iter()
        .map(|elf_food| elf_food.iter().sum::<i32>())
        .collect();

    result.sort();
    result.reverse();

    let part_1_result = result.get(0).unwrap();
    println!("A result {:?}", part_1_result);

    let part_2_result = &result[0..3].iter().sum::<i32>();
    println!("B result {:?}", part_2_result);
}
fn main() {
    day_01()
}
