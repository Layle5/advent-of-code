fn main() {
    let input = include_str!("input.txt");
    let grid: Vec<&[u8]> = input.lines().map(str::trim).map(str::as_bytes).collect();

    let mut sum: u64 = 0;
    let mut gear_ratios: u64 = 0;
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &byte) in row.iter().enumerate() {
            if byte.is_ascii_digit() || byte == b'.' { continue; }

            let top_parts = find_row_parts(&grid, row_index - 1, col_index);
            let middle_parts = find_row_parts(&grid, row_index, col_index);
            let bottom_parts = find_row_parts(&grid, row_index + 1, col_index);

            let parts: Vec<u64> = top_parts.into_iter()
                .chain(middle_parts.into_iter())
                .chain(bottom_parts.into_iter())
                .filter_map(|o| o)
                .map(|n| n.value)
                .collect();

            sum += parts.iter().sum::<u64>();
            if byte == b'*' && parts.len() == 2 {
                gear_ratios += parts.iter().product::<u64>()
            }
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", gear_ratios);
}

#[derive(Debug)]
struct Number {
    value: u64,
    end_col_index: usize,
}

fn find_row_parts(grid: &[&[u8]], row_index: usize, col_index: usize) -> [Option<Number>; 3] {
    let left = find_part_number(&grid, row_index, col_index - 1);
    let middle = if left.is_some() { None } else { find_part_number(&grid, row_index, col_index) };
    let check_right = match (&left, &middle) {
        (Some(n), _) => { n.end_col_index < col_index + 1 }
        (_, Some(_)) => false,
        (None, None) => true,
    };
    let right = if check_right { find_part_number(&grid, row_index, col_index + 1) } else { None };
    [left, middle, right]
}

fn find_part_number(grid: &[&[u8]], row_index: usize, col_index: usize) -> Option<Number> {
    let row = grid[row_index];

    let mut value = 0;
    let mut start_col_index = col_index;
    let mut end_col_index = start_col_index;

    while end_col_index < row.len() &&
        row[end_col_index].is_ascii_digit() {
        value = value * 10 + (row[end_col_index] - b'0') as u64;
        end_col_index += 1;
    }

    if start_col_index == end_col_index { return None; }

    while start_col_index > 0 && row[start_col_index - 1].is_ascii_digit() {
        start_col_index -= 1;
        let power_of_ten = 10u64.pow((end_col_index - start_col_index - 1) as u32);
        value += (row[start_col_index] - b'0') as u64 * power_of_ten;
    }

    Some(Number {
        value,
        end_col_index,
    })
}
