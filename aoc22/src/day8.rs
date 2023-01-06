use std::collections::HashSet;

fn get_scenic_score(tree_matrix: &[Vec<u32>], row: &usize, col: &usize) -> usize {
    let cols = tree_matrix[0].len();
    let val = tree_matrix[*row][*col];

    let mut north_count = 0;
    for i in (0..*row).rev() {
        north_count += 1;
        if val > tree_matrix[i][*col] {
            continue;
        }
        break;
    }
    let mut south_count = 0;
    for tree_row in tree_matrix.iter().skip(*row + 1) {
        south_count += 1;
        if val > tree_row[*col] {
            continue;
        }
        break;
    }

    let mut east_count = 0;
    for j in (0..*col).rev() {
        east_count += 1;
        if val > tree_matrix[*row][j] {
            continue;
        }
        break;
    }
    let mut west_count = 0;
    for j in *col + 1..cols {
        west_count += 1;
        if val > tree_matrix[*row][j] {
            continue;
        }
        break;
    }
    north_count * south_count * east_count * west_count
}

pub fn solution() {
    let tree_matrix: Vec<Vec<u32>> = include_str!("../data/day8.txt")
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let rows = tree_matrix.len();
    let cols = tree_matrix[0].len();
    let mut counted_trees = HashSet::new();
    let mut visible_trees = rows * 2 + (cols - 2) * 2;
    visible_trees += tree_matrix
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < rows - 1 && *i != 0)
        .fold(0, |mut acc, (i, row)| {
            for j in 1..cols - 1 {
                if counted_trees.get(&(i, j)).is_some() {
                    continue;
                }
                if tree_matrix[i][j] > *row[0..j].iter().max().unwrap() {
                    acc += 1;
                    counted_trees.insert((i, j));
                };
            }
            for j in (1..cols - 1).rev() {
                if counted_trees.get(&(i, j)).is_some() {
                    continue;
                }
                if tree_matrix[i][j] > *row[j + 1..cols].iter().max().unwrap() {
                    acc += 1;
                    counted_trees.insert((i, j));
                };
            }
            acc
        });

    for j in 1..cols - 1 {
        for i in 1..rows - 1 {
            if counted_trees.get(&(i, j)).is_some() {
                continue;
            }
            if tree_matrix[i][j] > tree_matrix[0..i].iter().map(|col| col[j]).max().unwrap() {
                visible_trees += 1;
                counted_trees.insert((i, j));
            };
        }
        for i in (1..rows - 1).rev() {
            if counted_trees.get(&(i, j)).is_some() {
                continue;
            }
            if tree_matrix[i][j]
                > tree_matrix[i + 1..rows]
                    .iter()
                    .map(|col| col[j])
                    .max()
                    .unwrap()
            {
                visible_trees += 1;
                counted_trees.insert((i, j));
            };
        }
    }

    let mut max_scenic_score = 0;
    for (i, j) in counted_trees.iter() {
        let sc = get_scenic_score(&tree_matrix, i, j);
        if sc > max_scenic_score {
            max_scenic_score = sc;
        }
    }
    println!("There are {visible_trees} visible trees from outside the grid");
    println!("The highest scenic score possible is {max_scenic_score}");
}
