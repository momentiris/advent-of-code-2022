use std::fs;

#[derive(Debug)]
pub struct Instruction {
    from: i32,
    to: i32,
    amount: i32,
}

pub fn make() {
    let raw_input = fs::read_to_string("input/day-05.txt").expect("Failed to read input");

    let stack_lines: Vec<Vec<&str>> = raw_input
        .lines()
        .take(8)
        // .take(3)
        .map(|x| x.split("").collect())
        .collect();

    let instructions: Vec<Instruction> = raw_input
        .lines()
        .skip(10)
        // .skip(5)
        .map(to_instruction)
        .collect();

    let lane_index_raw: Vec<&str> = raw_input
        .lines()
        .skip(8)
        // .skip(3)
        .take(1)
        .map(|x| x.split("").collect())
        .last()
        .unwrap();
    println!("Day 5 results");

    let lanes = get_as_lanes(lane_index_raw, stack_lines);

    println!("pt 1: {:?}", part_2(instructions, lanes));
}

pub fn part_1(instructions: Vec<Instruction>, lanes: Vec<Vec<&str>>) {
    let after = handle_instructions(instructions, lanes);
    let result: Vec<&&str> = after.iter().map(|x| x.get(0).unwrap()).collect();

    println!("pt 1: {:?}", result);
}

pub fn part_2(instructions: Vec<Instruction>, lanes: Vec<Vec<&str>>) {
    let after = handle_instructions_2(instructions, lanes);
    let result: Vec<&&str> = after.iter().map(|x| x.get(0).unwrap()).collect();

    println!("pt 1: {:?}", result);
}

pub fn handle_instructions(
    instructions: Vec<Instruction>,
    stacks: Vec<Vec<&str>>,
) -> Vec<Vec<&str>> {
    let mut acc: Vec<Vec<&str>> = stacks.to_owned();

    for instruction in instructions {
        println!("inst: {:?}", instruction);
        let from = usize::try_from(instruction.from).unwrap() - 1;
        let to = usize::try_from(instruction.to).unwrap() - 1;
        let amount = usize::try_from(instruction.amount).unwrap();

        let mut removed: Vec<_> = acc[from].drain(0..amount).collect();

        acc[to].reverse();
        acc[to].append(&mut removed);
        acc[to].reverse();
    }

    return acc;
}

pub fn handle_instructions_2(
    instructions: Vec<Instruction>,
    stacks: Vec<Vec<&str>>,
) -> Vec<Vec<&str>> {
    let mut acc: Vec<Vec<&str>> = stacks.to_owned();

    for instruction in instructions {
        let from = usize::try_from(instruction.from).unwrap() - 1;
        let to = usize::try_from(instruction.to).unwrap() - 1;
        let amount = usize::try_from(instruction.amount).unwrap();
        let removed: Vec<_> = acc[from].drain(0..amount).collect();
        acc[to] = [removed.as_slice(), acc[to].as_slice()].concat();
    }

    return acc;
}

pub fn get_as_lanes<'a>(lanes: Vec<&'a str>, stack_lines: Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let lanes_clean: Vec<&str> = lanes
        .iter()
        .copied()
        .filter(|x| x != &"" && x != &" ")
        .collect();

    let mut v: Vec<Vec<&str>> = vec![];

    lanes_clean.iter().for_each(|_x| v.push(vec![]));
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
