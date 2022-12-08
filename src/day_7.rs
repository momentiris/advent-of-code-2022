use std::fs;

pub enum Command<'a> {
    Ls(Vec<&'a str>),
    Cd(&'a str),
}

pub fn make() {
    let raw_input = fs::read_to_string("input/day-07.sample.txt").expect("Failed to read input");
    let input: Vec<Vec<&str>> = raw_input
        .split("$ ")
        .map(|x| x.split("\n").filter(|x| !x.is_empty()).collect())
        .filter(|x: &Vec<&str>| !x.is_empty())
        .collect();

    let foo = input.iter().cloned().map(make_command);
    println!("Day 7 results");

    // let ok = part_1(4usize, raw_input);
    println!("pt 2: {:?}", input);
}

pub fn make_command(a: Vec<&str>) -> Command {
    let ok = a.get(0).copied().unwrap().chars().collect::<Vec<char>>();
    match ok {
        ['a', 'b'] => true,
        _ => true,
    }
}

pub fn part_1(input: Vec<char>) -> usize {
    2
}

pub fn part_2(input: Vec<char>) -> usize {
    2
}
