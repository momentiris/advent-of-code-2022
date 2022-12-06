use std::fs;

pub fn make() {
    let raw_input = fs::read_to_string("input/day-06.txt").expect("Failed to read input");

    println!("Day 6 results");
    println!("pt 1: {:?}", part_1(raw_input.chars().collect()));
    println!("pt 2: {:?}", part_2(raw_input.chars().collect()));
}

pub fn part_1(input: Vec<char>) -> usize {
    let mut i = 0;

    input.iter().enumerate().for_each(|(a, _b)| {
        if i != 0 {
            return;
        }

        if is_packet(a, input.clone()) == true {
            i = a;
        }
    });

    return i;
}

pub fn part_2(input: Vec<char>) -> usize {
    let mut i = 0;

    input.iter().enumerate().for_each(|(a, _b)| {
        if i != 0 {
            return;
        }

        if is_message(a, input.clone()) == true {
            i = a;
        }
    });

    return i;
}

pub fn is_packet(index: usize, list: Vec<char>) -> bool {
    if index < 4 {
        return false;
    };

    return match list.get(index - 4..index) {
        Some(slice) => unique_chars(slice.to_vec()),
        None => false,
    };
}

pub fn is_message(index: usize, list: Vec<char>) -> bool {
    if index < 14 {
        return false;
    };

    return match list.get(index - 14..index) {
        Some(slice) => unique_chars(slice.to_vec()),
        None => false,
    };
}

fn unique_chars(s: Vec<char>) -> bool {
    let mut y = s.clone();
    y.sort();
    y.dedup();
    s.len() == y.len()
}
