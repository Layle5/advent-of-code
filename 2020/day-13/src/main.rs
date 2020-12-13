use std::env;
use std::fs;

use num_bigint::BigInt;

fn first_bigger_multiple(n: usize, target: usize) -> usize {
    ((target + n - 1) / n) * n
}

fn solve_part_1(content: &str) {
    let mut lines = content.lines();
    let estimated_time: usize = lines.next().unwrap().parse().unwrap();
    let (bus_id, delta_time) = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|bus_id_str| *bus_id_str != "x")
        .map(str::parse)
        .map(Result::unwrap)
        .map(|bus_id| (bus_id, first_bigger_multiple(bus_id, estimated_time)))
        .map(|(bus_id, next_stop)| (bus_id, next_stop - estimated_time))
        .min_by_key(|(_, delta_time)| *delta_time)
        .unwrap();

    println!("Part 1: {}", bus_id * delta_time);
}

fn bezout_identity(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    if b.eq(&BigInt::from(1)) {
        return (BigInt::from(0), BigInt::from(1));
    }

    let q = a / b;
    let r = a % b;
    let (x1, x2) = bezout_identity(b, &r);
    // 1 = b * x1 + r * x2
    //   = b * x1 + (a - (q * b)) * x2
    //   = b * x1 + a * x2 - (q * b * x2)
    //   = a * x2 + b * (x1 - q * x2)
    (x2.clone(), x1 - q * x2)
}

fn solve_part_2(content: &str) {
    let mut lines = content.lines();
    lines.next();
    let mut bus_ids: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, bus_id_str)| *bus_id_str != "x")
        .map(|(index, bus_id_str)| (index, bus_id_str.parse().unwrap()))
        .collect();

    bus_ids.sort_by_key(|(_, bus_id)| *bus_id);
    bus_ids.reverse();

    let mut r = BigInt::from(bus_ids[0].0);
    let mut q = BigInt::from(bus_ids[0].1);
    let mut index = 1;
    while index < bus_ids.len() {
        let a1 = r;
        let a2 = BigInt::from(bus_ids[index].0);
        let n1 = q;
        let n2 = BigInt::from(bus_ids[index].1);
        let (m1, m2) = bezout_identity(&n1, &n2);
        q = n1.clone() * n2.clone();
        r = (a1 * m2 * n2) % q.clone() + (a2 * m1 * n1) % q.clone();
        r = ((r % q.clone()) + q.clone()) % q.clone();
        index += 1;
    }

    println!("Part 2: {}", q - r);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve_part_1(&content);
    solve_part_2(&content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let assert = |a, b, e: (isize, isize)| {
            let ba = BigInt::from(a);
            let bb = BigInt::from(b);
            let be = (BigInt::from(e.0), BigInt::from(e.1));
            assert_eq!(bezout_identity(&ba, &bb), be);
        };
        assert(2, 1, (0, 1));
        assert(5, 2, (1, -2));
        assert(7, 5, (-2, 3));
        assert(2007, 7, (3, -860));
        assert(2014, 2007, (-860, 863));
        assert(4021, 2014, (863, -1723));
    }
}
