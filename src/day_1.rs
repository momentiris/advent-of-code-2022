use std::fs;

pub fn make() {
    let raw_input = fs::read_to_string("input/day-01.txt").expect("Failed to read input");

    let elves: Vec<Vec<i32>> = raw_input
        .split("\n\n")
        .map(|x: &str| x.split("\n").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut calories: Vec<i32> = elves.iter().map(|elf| elf.iter().sum()).collect();

    calories.sort();
    calories.reverse();

    println!("Day 1 results");
    let part_1_result = calories.get(0).unwrap();
    println!("pt 1: {:?}", part_1_result);

    let part_2_result = calories.iter().take(3).sum::<i32>();
    println!("pt 2: {:?}", part_2_result);
}
