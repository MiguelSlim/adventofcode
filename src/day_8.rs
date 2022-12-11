use std::collections::HashSet;

use crate::read_lines;

type TreeGrid = Vec<Vec<u8>>;

pub fn day_8_part_1() {
    let mut grid: TreeGrid = Vec::new();
    let mut visible_trees = 0;

    if let Ok(lines) = read_lines("./inputs/day_8.txt") {
        for line in lines {
            let mut row = Vec::<u8>::new();
            if let Ok(item) = line {
                for tree in item.chars() {
                    let num = tree.to_string().parse::<u8>().unwrap();
                    row.push(num);
                    visible_trees += 1;
                }
            }
            grid.push(row);
        }
    }

    for i in 1..grid.len() - 1 {
        let row = &grid[i];
        for j in 1..row.len() - 1 {
            if is_hidden(&grid, (i, j)) {
                visible_trees -= 1;
            }
        }
    }

    println!("visible_trees: {}", visible_trees);
}

fn is_hidden(grid: &TreeGrid, position: (usize, usize)) -> bool {
    let (row_idx, col_idx) = position;
    let tree = grid[row_idx][col_idx];

    let mut hidden_left = false;
    for i in 0..col_idx {
        if grid[row_idx][i] >= tree {
            hidden_left = true;
            break;
        }
    }
    let mut hidden_right = false;
    for i in (col_idx + 1)..grid[row_idx].len() {
        if grid[row_idx][i] >= tree {
            hidden_right = true;
            break;
        }
    }
    let mut hidden_above = false;
    for i in 0..row_idx {
        if grid[i][col_idx] >= tree {
            hidden_above = true;
            break;
        }
    }
    let mut hidden_below = false;
    for i in (row_idx + 1)..grid.len() {
        if grid[i][col_idx] >= tree {
            hidden_below = true;
            break;
        }
    }

    hidden_left && hidden_right && hidden_above && hidden_below
}

pub fn day_8_part_2() {
    let mut grid: TreeGrid = Vec::new();

    if let Ok(lines) = read_lines("./inputs/day_8.txt") {
        for line in lines {
            let mut row = Vec::<u8>::new();
            if let Ok(item) = line {
                for tree in item.chars() {
                    let num = tree.to_string().parse::<u8>().unwrap();
                    row.push(num);
                }
            }
            grid.push(row);
        }
    }

    let mut best_scenic_score = 0;
    for i in 1..grid.len() - 1 {
        let row = &grid[i];
        for j in 1..row.len() - 1 {
            let scenic_score = calc_scenic_score(&grid, (i, j));
            best_scenic_score = best_scenic_score.max(scenic_score);
        }
    }

    println!("best scenic score: {}", best_scenic_score);
}

fn calc_scenic_score(grid: &TreeGrid, position: (usize, usize)) -> i32 {
    let (row_idx, col_idx) = position;
    let tree = grid[row_idx][col_idx];

    let mut score_left = 0;
    for i in 0..col_idx {
        score_left += 1;
        if grid[row_idx][i] >= tree {
            score_left = 1;
        }
    }
    let mut score_right = 0;
    for i in (col_idx + 1)..grid[row_idx].len() {
        score_right += 1;
        if grid[row_idx][i] >= tree {
            break;
        }
    }
    let mut score_above = 0;
    for i in 0..row_idx {
        score_above += 1;
        if grid[i][col_idx] >= tree {
            score_above = 1;
        }
    }
    let mut score_below = 0;
    for i in (row_idx + 1)..grid.len() {
        score_below += 1;
        if grid[i][col_idx] >= tree {
            break;
        }
    }

    score_left * score_right * score_above * score_below
}
