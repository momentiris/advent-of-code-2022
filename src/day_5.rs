use std::fs;

#[derive(Debug)]
pub struct Instruction {
    from: i32,
    to: i32,
    amount: i32,
}

pub fn make() {
    let raw_input = fs::read_to_string("input/day-05.sample.txt").expect("Failed to read input");

    println!("Day 4 results");
    println!("pt 1: {:?}", part_1(&raw_input));
}

pub fn part_1(input: &String) -> i32 {
    let foo: Vec<&str> = input.split_inclusive("]\n").collect();
    let stack_lines: Vec<Vec<&str>> = foo
        .get(0)
        .unwrap()
        .lines()
        .map(|x| x.split("").collect())
        .collect();

    let instructions: Vec<Instruction> = foo
        .get(1)
        .unwrap()
        .split("\n\n")
        .last()
        .unwrap()
        .split("\n")
        .map(to_instruction)
        .collect();

    let ok: Vec<&str> = foo.get(1).unwrap().split("\n\n").collect();
    let num_lanes = ok.get(0).copied().unwrap().split("").collect();

    let lanes = get_as_lanes(num_lanes, stack_lines);

    println!("lanes: {:?}", lanes);
    println!("instructions: {:?}", instructions);
    2
}

pub fn get_as_lanes<'a>(lanes: Vec<&'a str>, stack_lines: Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut v: Vec<Vec<&str>> = vec![vec![], vec![], vec![]];

    for stack_line in stack_lines.iter() {
        for (pos, e) in stack_line.iter().copied().enumerate() {
            let lanes_pos_item = lanes.get(pos).copied().unwrap().trim();

            if lanes_pos_item != "" && lanes_pos_item != " " && e != "" && e != " " {
                v[lanes_pos_item.parse::<usize>().unwrap() - 1].push(e);
            }
        }
    }

    return v;
}

// pub fn to_stacks(s: Vec<Vec<&str>>) -> Vec<&str> {
//     println!("stack before: {:?}", s);
//     let mut foo: Vec<Vec<&str>> = Vec::new();

//     for (pos, e) in s.iter().enumerate() {
//         println!("element {:?} in pos {:?}", pos, e);
//         for (pos, e) in s.iter().enumerate() {
//             println!("element {:?} in pos {:?}", pos, e);
//         }
//     }

//     println!("stack after: {:?}", foo);
//     return vec!["s"];
// }

pub fn to_instruction(s: &str) -> Instruction {
    let v: Vec<_> = s
        .split(" ")
        .filter(|str| str.parse::<i32>().is_ok())
        .collect();

    return Instruction {
        from: v.get(1).unwrap().parse().unwrap(),
        to: v.get(2).unwrap().parse().unwrap(),
        amount: v.get(0).unwrap().parse().unwrap(),
    };
}
