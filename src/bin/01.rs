use std::collections::HashMap;
use std::iter::zip;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|x: &str| x.split("   ").collect())
        .map(|x: Vec<&str>| (x[0].parse::<u64>().unwrap(), x[1].parse::<u64>().unwrap()))
        .fold((vec![], vec![]), |mut acc: (Vec<u64>, Vec<u64>), x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let r: (Vec<u64>, Vec<u64>) = {
        let mut t = parse_input(input);
        t.1.sort();
        t.0.sort();
        t
    };

    let z = zip(r.0.iter().map(|x| *x as i64), r.1.iter().map(|x| *x as i64));

    Some(z.fold(0, |acc, items| acc + (items.0 - items.1).abs() as u64))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some({
        let (left, right) = parse_input(input);
        let mut counters : HashMap<u64, u64> = HashMap::new();
        for v in right {
            if counters.contains_key(&v) {
                *counters.get_mut(&v).unwrap() += 1;
            } else {
                counters.insert(v, 1);
            }
        }
        left.iter().fold(0, |acc, x| acc + (counters.get(x).unwrap_or(&0) * x))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
