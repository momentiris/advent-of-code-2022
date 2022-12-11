use std::fs;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
    Unknown,
}

pub fn make() {
    let raw_input = fs::read_to_string("input/day-10.sample.txt").expect("Failed to read input");

    let input = raw_input
        .lines()
        .map(|x| x.split(" ").collect())
        .map(to_instruction)
        .collect::<Vec<Instruction>>();

    // println!("pt 1 {:?}", part_1(input));
    println!("pt 2 {:?}", part_2(input));
    // println!("pt 2 {:?}", part_2(input_2))
}

pub fn _part_1(program: Vec<Instruction>) -> i32 {
    let mut x = 1;
    let mut cycles: Vec<i32> = vec![];
    let mut during_cycles: Vec<i32> = vec![];

    for instruction in program.iter() {
        match instruction {
            Instruction::Noop | Instruction::Unknown => {
                cycles.push(x);
                during_cycles.push(x);
            }
            Instruction::Addx(value) => {
                for index in [0, 1].iter() {
                    during_cycles.push(x);
                    if index == &1 {
                        x += value;
                        cycles.push(x)
                    } else {
                        cycles.push(x)
                    }
                }
            }
        }
    }

    let ok = [20, 60, 100, 140, 180, 220]
        .map(|x| return during_cycles.get(x - 1).unwrap() * i32::try_from(x).unwrap())
        .iter()
        .sum::<i32>();

    return ok;
}

pub fn part_2(program: Vec<Instruction>) -> i32 {
    let mut x = 1;
    let mut cycles: Vec<i32> = vec![0];
    let mut during_cycles: Vec<i32> = vec![];

    let mut screen = [["."; 240]; 6];

    let mut sprite_position = vec![0, 1, 2];

    for instruction in program.iter() {
        println!("\n");

        match instruction {
            Instruction::Noop | Instruction::Unknown => {
                println!("during cycle {:?} x is {:?}", cycles.len(), x);
                let current_row = ((cycles.len() - 1) / 40) + 1;

                let cycle_as_num = i32::try_from(cycles.len() - 1).unwrap();
                println!(
                    "sprite position {:?} cycle_as_num is {:?}",
                    sprite_position, cycle_as_num
                );

                let overlaps = sprite_position.iter().any(|x| x == &cycle_as_num);
                if overlaps {
                    screen[0][cycles.len() - 1] = "#";
                } else {
                    screen[0][cycles.len() - 1] = ".";
                }
                cycles.push(x);
                during_cycles.push(x);
            }
            Instruction::Addx(value) => {
                for index in [0, 1].iter() {
                    during_cycles.push(x);

                    if index == &1 {
                        println!("during cycle {:?} x is {:?}", cycles.len(), x);
                        x += value;
                        println!("end cycle {:?} x is {:?}", cycles.len(), x);

                        println!("sprite position {:?}", sprite_position);
                        // println!("after cycle {:?} x is {:?}", cycle, x);
                        // let current_row = ((cycles.len() - 1) / 40) + 1;
                        let cycle_as_num = i32::try_from(cycles.len() - 1).unwrap();
                        println!(
                            "sprite position {:?} cycle_as_num is {:?}",
                            sprite_position, cycle_as_num
                        );
                        let overlaps = sprite_position.iter().any(|x| x == &cycle_as_num);
                        if overlaps {
                            screen[0][cycles.len() - 1] = "#";
                        } else {
                            screen[0][cycles.len() - 1] = ".";
                        }

                        cycles.push(x);
                        sprite_position = vec![x - 1, x, x + 1];
                    } else {
                        let cycle_as_num = i32::try_from(cycles.len() - 1).unwrap();
                        // let current_row = ((cycles.len() - 1) / 40) + 1;

                        // println!("current row is {:?}", cycles.len());
                        let overlaps = sprite_position.iter().any(|x| x == &cycle_as_num);

                        if overlaps {
                            screen[0][cycles.len() - 1] = "#";
                        } else {
                            screen[0][cycles.len() - 1] = ".";
                        }

                        cycles.push(x);

                        println!(
                            "sprite position {:?} cycle_as_num is {:?}",
                            sprite_position, cycle_as_num
                        );
                        // let current_row = ((cycles.len() - 1) / 40) + 1;
                    }
                }
            }
        }
    }

    println!("screen {:?}", screen);

    let ok = [20, 60, 100, 140, 180, 220]
        .map(|x| return during_cycles.get(x - 1).unwrap() * i32::try_from(x).unwrap())
        .iter()
        .sum::<i32>();

    return ok;
}

fn to_instruction(x: Vec<&str>) -> Instruction {
    match x.get(0).unwrap().clone() {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Addx(x.get(1).unwrap().parse().unwrap()),
        _ => Instruction::Unknown,
    }
}
