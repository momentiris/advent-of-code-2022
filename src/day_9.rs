use std::fs;

type Cell = (i32, i32);

pub fn make() {
    let raw_input = fs::read_to_string("input/day-09.sample.txt").expect("Failed to read input");

    let input = raw_input
        .lines()
        .map(|x| x.split(" ").collect())
        .collect::<Vec<Vec<&str>>>();

    let input_2 = input.clone();
    // println!("pt 1 {:?}", part_1(input));
    println!("pt 2 {:?}", part_2(input_2))
}

pub fn part_1(instructions: Vec<Vec<&str>>) -> usize {
    let mut head: Cell = (0, 0);
    let mut tail: Cell = (0, 0);
    let mut tail_steps: Vec<Cell> = vec![];

    for instruction_raw in instructions.iter() {
        let direction = instruction_raw.get(0).unwrap();
        let steps = instruction_raw.get(1).unwrap().parse::<usize>().unwrap();
        let steps_range = 0..steps;

        for _ in steps_range {
            head = make_step(direction, head);
            tail = follow_step(head, tail);
            tail_steps.push(tail);
        }
    }
    tail_steps.sort();

    tail_steps.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    return tail_steps.len();
}

pub fn part_2(instructions: Vec<Vec<&str>>) -> usize {
    let mut rope = vec![(0, 0); 9];
    let mut foo_rope = rope.clone();

    let mut head: Cell = (0, 0);
    let mut tail: Cell;
    let mut tail_steps: Vec<Cell> = vec![];

    for instruction_raw in instructions.iter() {
        let direction = instruction_raw.get(0).unwrap();
        let steps = instruction_raw.get(1).unwrap().parse::<usize>().unwrap();
        let steps_range = 0..steps;

        for _ in steps_range {
            head = make_step(direction, head);
            println!("head {:?}", head);
            let mut next_head: Cell = head;
            // tail_steps.push(tail);
            let mut ok_foo_rope: Vec<(i32, i32)> = vec![];
            let mut foo_next_head: (i32, i32);
            for r in rope.iter() {
                let asdf = ok_foo_rope.last().unwrap().to_owned();
                next_head = follow_step(next_head, asdf);
                ok_foo_rope.push(next_head);

                println!("next head {:?}", next_head);
            }

            // rope = foo_rope;
        }
    }

    // let mut ok: Vec<_> = tail_steps.iter().skip(9).step_by(9).collect();
    // ok.sort();

    // ok.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    return tail_steps.len();
}

pub fn follow_step(head: Cell, tail: Cell) -> Cell {
    return match (head.0 - tail.0, head.1 - tail.1) {
        (2, 0) => (tail.0 + 1, tail.1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (0, 2) => (tail.0, tail.1 + 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (1, 2) | (2, 1) => (tail.0 + 1, tail.1 + 1),
        (-1, -2) | (-2, -1) => (tail.0 - 1, tail.1 - 1),
        (-1, 2) | (-2, 1) => (tail.0 - 1, tail.1 + 1),
        (1, -2) | (2, -1) => (tail.0 + 1, tail.1 - 1),
        _ => tail,
    };
}

pub fn make_step(direction: &str, (x, y): Cell) -> Cell {
    return match direction {
        "R" => (x + 1, y),
        "L" => (x - 1, y),
        "U" => (x, y + 1),
        "D" => (x, y - 1),
        _ => (x, y),
    };
}
