use std::{
    cmp::{max, min},
    fs,
};

pub type Assignment = (i32, i32);
pub type AssignmentPair = (Assignment, Assignment);

pub fn make() {
    let raw_input = fs::read_to_string("input/day-04.txt").expect("Failed to read input");

    println!("Day 4 results");
    println!("pt 1: {:?}", part_1(&raw_input));
    println!("pt 2: {:?}", part_2(&raw_input));
}

pub fn part_1(input: &String) -> i32 {
    let parsed_input: Vec<Vec<&str>> = input.lines().map(|x| x.split(",").collect()).collect();
    let ranges: Vec<AssignmentPair> = parsed_input
        .iter()
        .cloned()
        .map(to_assignment_pair)
        .collect();

    let result: Vec<AssignmentPair> = ranges.iter().cloned().filter(overlaps_fully).collect();
    return result.len().try_into().unwrap();
}

pub fn part_2(input: &String) -> i32 {
    let parsed_input: Vec<Vec<&str>> = input.lines().map(|x| x.split(",").collect()).collect();
    let ranges: Vec<AssignmentPair> = parsed_input
        .iter()
        .cloned()
        .map(to_assignment_pair)
        .collect();

    let result: Vec<AssignmentPair> = ranges.iter().cloned().filter(overlaps).collect();
    return result.len().try_into().unwrap();
}

pub fn to_assignment(x: &str) -> Assignment {
    let p: Vec<i32> = x.split("-").map(|x| x.parse::<i32>().unwrap()).collect();

    return (p.get(0).unwrap().clone(), p.get(1).unwrap().clone());
}

pub fn to_assignment_pair(x: Vec<&str>) -> AssignmentPair {
    let a = x.get(0).cloned().unwrap();
    let b = x.get(1).cloned().unwrap();

    return (to_assignment(a), to_assignment(b));
}

pub fn overlaps_fully((a, b): &AssignmentPair) -> bool {
    let a_contains_b = a.0 <= b.0 && a.1 >= b.1;
    let b_contains_a = b.0 <= a.0 && b.1 >= a.1;

    return a_contains_b || b_contains_a;
}

pub fn overlaps((a, b): &AssignmentPair) -> bool {
    return max(a.0, b.0) <= min(b.1, a.1);
}
