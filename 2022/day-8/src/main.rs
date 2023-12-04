use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let tree_grid = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .map(|bytes| bytes.iter().map(|byte| byte - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let number_rows = tree_grid.len();
    let number_cols = tree_grid[0].len();

    let mut count = number_rows * 2 + (number_cols - 2) * 2;
    for row_index in 1..number_rows-1 {
        for col_index in 1..number_cols - 1 {
            let ct = tree_grid[row_index][col_index];
            let is_visible =
                tree_grid[row_index][0..col_index].iter().all(|t| *t < ct) ||
                tree_grid[row_index][col_index + 1..].iter().all(|t| *t < ct) ||
                tree_grid[0..row_index].iter().all(|v| v[col_index] < ct) ||
                tree_grid[row_index + 1..].iter().all(|v| v[col_index] < ct);

            if is_visible {
                count += 1;
            }
        }
    }

    println!("Part 1: {count}");

    let mut best: Option<usize> = None;
    for row_index in 1..number_rows-1 {
        for col_index in 1..number_cols - 1 {
            let ct = tree_grid[row_index][col_index];
            let scenic_score =
                count_scenic_score(ct, tree_grid[row_index][0..col_index].iter().rev().copied()) *
                count_scenic_score(ct, tree_grid[row_index][col_index + 1..].iter().copied()) *
                count_scenic_score(ct, tree_grid[0..row_index].iter().rev().map(|v| v[col_index])) *
                count_scenic_score(ct, tree_grid[row_index + 1..].iter().map(|v| v[col_index]));

            best = best.max(Some(scenic_score));
        }
    }

    println!("Part 2: {}", best.unwrap());
}

fn count_scenic_score<I: Iterator<Item = u8>>(current_tree: u8, mut iterator: I) -> usize {
    iterator.fold_while(0usize, |count, tree| {
        if current_tree <= tree {
            itertools::FoldWhile::Done(count + 1)
        } else {
            itertools::FoldWhile::Continue(count + 1)
        }
    }).into_inner()
}