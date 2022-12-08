use std::fs;

pub fn make() {
    let raw_input = fs::read_to_string("input/day-08.sample.txt").expect("Failed to read input");
    let input = raw_input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let input_2 = input.clone();

    println!("Day 7 results");
    println!("pt 1: {:?}", part_1(input));
    println!("pt 2: {:?}", part_2(input_2));
}

pub fn part_1(map: Vec<Vec<char>>) -> usize {
    let mut visible_trees: Vec<char> = vec![];

    for (row_index, row) in map.iter().enumerate() {
        for (col_index, tree) in row.iter().enumerate() {
            match is_edge(row_index, col_index, map.len() - 1, row.len() - 1) {
                true => visible_trees.push(*tree),
                false => match (
                    is_tree_row_visible(get_col(&map, col_index), row_index),
                    is_tree_row_visible(row.clone(), col_index),
                ) {
                    (false, false) => (),
                    _ => visible_trees.push(*tree),
                },
            }
        }
    }

    return visible_trees.len();
}

pub fn part_2(map: Vec<Vec<char>>) -> usize {
    let mut neighbors_score: Vec<usize> = vec![];

    for (row_index, row) in map.iter().enumerate() {
        for (col_index, tree) in row.iter().enumerate() {
            let col = get_col(&map, col_index);
            neighbors_score.push(get_neighbors_score(row, col, row_index, col_index, tree));
        }
    }
    neighbors_score.sort();
    neighbors_score.reverse();

    return *neighbors_score.get(0).unwrap();
}

pub fn get_sum_good_neighbor(t: &char, neighbors: Vec<char>) -> usize {
    let tree = t.to_string().parse::<usize>().unwrap();
    let mut good_neighbors = 0usize;

    for neighbor in neighbors.iter().copied() {
        let neighbor = neighbor.to_string().parse::<usize>().unwrap();
        if tree <= neighbor {
            good_neighbors += 1;
            break;
        }

        if tree >= neighbor {
            good_neighbors += 1;
        }
    }

    good_neighbors
}

pub fn get_neighbors_score(
    row: &Vec<char>,
    col: Vec<char>,
    col_index: usize,
    row_index: usize,
    tree: &char,
) -> usize {
    let (row_left, row_right) = (&row[..row_index], &row[row_index + 1..]);
    let (col_up, col_down) = (&col[..col_index], &col[col_index + 1..]);

    let mut rev_row_left = row_left.to_owned();
    let mut rev_col_up = col_up.to_owned();

    let row_right = row_right.to_owned();
    let col_down = col_down.to_owned();
    rev_row_left.reverse();
    rev_col_up.reverse();

    let binding = [rev_row_left, row_right, rev_col_up, col_down];

    let ok: Vec<usize> = binding
        .iter()
        .map(|neighbors| get_sum_good_neighbor(tree, neighbors.clone()))
        .collect();

    let foo = ok.iter().fold(1usize, |a, b| a * b);

    return foo;
}

pub fn get_col(map: &Vec<Vec<char>>, col_index: usize) -> Vec<char> {
    return map
        .iter()
        .map(|x| x.get(col_index).unwrap().clone())
        .collect::<Vec<char>>();
}

pub fn is_biggest(x: &[char], tree: &char) -> bool {
    return x
        .iter()
        .all(|x| x.to_string().parse::<i32>().unwrap() < tree.to_string().parse().unwrap());
}

pub fn is_tree_row_visible(row: Vec<char>, tree_index: usize) -> bool {
    let tree_in_row = row.get(tree_index).unwrap();

    return is_biggest(&row[..tree_index], tree_in_row)
        || is_biggest(&row[tree_index + 1..], tree_in_row);
}

pub fn is_edge(row_index: usize, col_index: usize, map_len: usize, row_len: usize) -> bool {
    if row_index == 0 || row_index == map_len {
        return true;
    } else if col_index == 0 || col_index == row_len {
        return true;
    }
    return false;
}
