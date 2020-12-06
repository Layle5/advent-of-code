use std::env;
use std::fs;

fn add_mod(n: usize, i: usize, m: usize) -> usize {
    return (n + i) % m;
}

fn solve_one(
    content: &str,
    row_increment: usize,
    col_increment: usize,
) -> usize {
    let map_tree_tile: u8 = b'#';
    let map: Vec<&[u8]> = content.lines().map(|line| line.as_bytes()).collect();

    let map_height = map.len();
    let map_width = map.first().unwrap().len();

    let mut tree_count: usize = 0;
    let mut position_row: usize = 0;
    let mut position_col: usize = 0;
    loop {
        position_row += row_increment;
        position_col = add_mod(position_col, col_increment, map_width);

        if map_height <= position_row {
            break;
        }

        let map_tile = map[position_row][position_col];
        if map_tile == map_tree_tile {
            tree_count += 1;
        }
    }

    tree_count
}

fn solve(content: &str, increments: &[(usize, usize)]) {
    let mut total_tree_product = 1;

    for &(down, right) in increments.iter() {
        let tree_count = solve_one(&content, down, right);
        total_tree_product *= tree_count;
        println!("Right {}, down {}: {}", right, down, tree_count);
    }

    println!("Total product: {}", total_tree_product);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let increments = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    solve(&content, &increments);
}

#[cfg(test)]
mod tests {}
