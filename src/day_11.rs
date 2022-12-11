use std::fs;

#[derive(Debug, Clone)]
pub struct Monkey<'a> {
    items: Vec<i64>,
    operation: (&'a str, &'a str),
    test: (&'a str, i64),
    success: usize,
    failure: usize,
    inspections: usize,
}

pub fn make() {
    let raw_input = fs::read_to_string("input/day-11.txt").expect("Failed to read input");

    let input = raw_input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.trim()).collect::<Vec<&str>>())
        .enumerate()
        .map(to_monkey)
        .collect::<Vec<Monkey>>();

    let input_2 = input.iter().cloned().collect::<Vec<Monkey>>();
    println!("pt 1 {:?}", part_1(input, 20, 3));
    println!("pt 2 {:?}", part_2(input_2, 10000));
}

pub fn part_2(m: Vec<Monkey>, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = m.iter().map(|x| x.clone()).clone().collect();
    let to_mod_with = m.iter().fold(1, |a, m| a * m.test.1);

    for _ in 0..rounds {
        for (index, monkey) in m.iter().enumerate() {
            let bar = monkeys.clone();
            let ok = bar[index].items.iter().clone();
            for mut item in ok {
                monkeys[index].inspections += 1;
                let new_worry_level = match monkey.operation {
                    ("*", "old") => item * item,
                    ("+", "old") => item + item,
                    ("*", _) => item * monkey.operation.1.parse::<i64>().unwrap(),
                    ("+", _) => item + monkey.operation.1.parse::<i64>().unwrap(),
                    ("-", _) => item - item,
                    _ => *item,
                };

                let new_worry_level = new_worry_level % to_mod_with;
                item = &new_worry_level;

                if item % monkey.test.1 == 0 {
                    monkeys[monkey.success].items.push(*item);
                } else {
                    monkeys[monkey.failure].items.push(*item);
                }
                monkeys[index].items.remove(0);
            }
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.reverse();

    return inspections.get(0).unwrap() * inspections.get(1).unwrap();
}

pub fn part_1(m: Vec<Monkey>, rounds: usize, worry_divider: i64) -> usize {
    let mut monkeys: Vec<Monkey> = m.iter().map(|x| x.clone()).clone().collect();
    for _ in 0..rounds {
        for (index, monkey) in m.iter().enumerate() {
            let bar = monkeys.clone();
            let ok = bar[index].items.iter().clone();
            for mut item in ok {
                monkeys[index].inspections += 1;

                let new_worry_level = match monkey.operation {
                    ("*", "old") => item * item,
                    ("+", "old") => item + item,
                    ("*", _) => item * monkey.operation.1.parse::<i64>().unwrap(),
                    ("+", _) => item + monkey.operation.1.parse::<i64>().unwrap(),
                    ("-", _) => item - item,
                    _ => *item,
                } / worry_divider;

                item = &new_worry_level;
                if item % monkey.test.1 == 0 {
                    monkeys[monkey.success].items.push(*item);
                } else {
                    monkeys[monkey.failure].items.push(*item);
                }
                monkeys[index].items.remove(0);
            }
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.reverse();

    inspections.get(0).unwrap() * inspections.get(1).unwrap()
}

pub fn to_monkey((_, x): (usize, Vec<&str>)) -> Monkey {
    let items = x[1]
        .split("Starting items: ")
        .map(|x| x.split(",").map(|x| x.trim()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let items = items[1]
        .iter()
        .map(|x| str::parse::<i64>(x).unwrap())
        .collect::<Vec<i64>>();

    let operation = x[2].split("Operation: ").collect::<Vec<&str>>()[1]
        .split("old ")
        .collect::<Vec<&str>>()[1]
        .split_at(1);

    let test = x[3].split("Test: ").collect::<Vec<&str>>()[1]
        .split(" by ")
        .collect::<Vec<&str>>();

    let success = x[4]
        .split("If true: throw to monkey ")
        .collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();

    let failure = x[5]
        .split("If false: throw to monkey ")
        .collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();

    // println!("failure {:?}.", failure);

    Monkey {
        items,
        operation: (operation.0, operation.1.trim()),
        test: (test[0], test[1].parse::<i64>().unwrap()),
        success,
        failure,
        inspections: 0,
    }
}
