use std::fs;

pub fn make() {
    let raw_input = fs::read_to_string("input/day-03.txt").expect("Failed to read input");
    println!("Day 3 results");
    println!("pt 1: {:?}", part_1(&raw_input));
    println!("pt 2: {:?}", part_2(&raw_input));
}

pub fn part_1(input: &String) -> i32 {
    let parsed_input: Vec<(String, String)> = input
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

    let common_chars: Vec<String> = parsed_input
        .iter()
        .cloned()
        .map(find_common_char_2)
        .collect();

    return common_chars.iter().cloned().map(get_point).sum::<i32>();
}

pub fn part_2(input: &String) -> i32 {
    let parsed_input: Vec<&str> = input.lines().collect();

    let common_chars: Vec<_> = parsed_input
        .chunks(3)
        .map(|slice| slice.to_owned())
        .map(|x| {
            (
                x.get(0).unwrap().to_string(),
                x.get(1).unwrap().to_string(),
                x.get(2).unwrap().to_string(),
            )
        })
        .collect();

    return common_chars
        .iter()
        .cloned()
        .map(find_common_char_3)
        .map(get_point)
        .sum::<i32>();
}

fn find_common_char_2<'a>((a, b): (String, String)) -> String {
    let hit: Vec<&str> = a
        .split("")
        .filter(|x| b.split("").any(|y| x.clone() == y))
        .collect();

    return hit.get(1).unwrap().to_string();
}

fn find_common_char_3<'a>((a, b, c): (String, String, String)) -> String {
    let hit: Vec<&str> = a
        .split("")
        .filter(|x| b.split("").any(|y| x.clone() == y) && c.split("").any(|z| x.clone() == z))
        .collect();

    return hit.get(1).unwrap().to_string();
}

fn get_point(s: String) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let char_index: i32 = alphabet.find(&s).unwrap().try_into().unwrap();
    char_index + 1
}
