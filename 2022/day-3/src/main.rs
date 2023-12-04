fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let sum: u64 = lines
        .iter()
        .map(|l| {
            let (left_str, right_str) = l.split_at(l.len() / 2);
            let mut left_bytes: Vec<u8> = left_str.as_bytes().iter().copied().collect();
            let mut right_bytes: Vec<u8> = right_str.as_bytes().iter().copied().collect();
            left_bytes.sort();
            right_bytes.sort();
            let mut left_index = 0;
            let mut right_index = 0;
            loop {
                let left_byte = left_bytes[left_index];
                let right_byte = right_bytes[right_index];
                if left_byte == right_byte {
                    return left_byte;
                } else if left_byte < right_byte {
                    left_index += 1;
                } else {
                    right_index += 1;
                }
            }
        })
        .map(|b| {
            if b.is_ascii_lowercase() {
                (b - b'a' + 1) as u64
            } else {
                (b - b'A' + 27) as u64
            }
        })
        .sum();

    println!("Part 1: {}", sum);

    let sum: u64 = lines
        .chunks(3)
        .map(|g| {
            dbg!(g);
            let mut bytes_array: [Vec<u8>; 3] = [
                g[0].as_bytes().iter().copied().collect(),
                g[1].as_bytes().iter().copied().collect(),
                g[2].as_bytes().iter().copied().collect(),
            ];
            bytes_array.iter_mut().for_each(|v| v.sort());
            let mut indexes: [usize; 3] = [0; 3];
            loop {
                let bytes: Vec<u8> = indexes
                    .iter()
                    .enumerate()
                    .map(|p| bytes_array[p.0][*p.1])
                    .collect();

                let mut keep_looping = false;
                for (i, bs) in bytes.windows(2).enumerate() {
                    let prev_byte = bs[0];
                    let next_byte = bs[1];
                    if prev_byte < next_byte {
                        keep_looping = true;
                        indexes[i] += 1;
                        break;
                    } else if prev_byte > next_byte {
                        keep_looping = true;
                        indexes[i + 1] += 1;
                        break;
                    }
                }

                if keep_looping {
                    continue;
                }

                return bytes[0];
            }
        })
        .map(|b| {
            if b.is_ascii_lowercase() {
                (b - b'a' + 1) as u64
            } else {
                (b - b'A' + 27) as u64
            }
        })
        .sum();

    println!("Part 2: {}", sum);
}
