use std::ops::{Range, RangeInclusive};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<usize>> = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap().into();
            let start = start.parse::<usize>().expect("Could not parse number");
            let end = end.parse::<usize>().expect("Could not parse number");
            RangeInclusive::new(start, end)
        })
        .collect_vec();

    let ingredients: Vec<usize> = ingredients
        .lines()
        .map(|i| i.parse::<usize>().expect("Could not parse number"))
        .collect();

    let count = ingredients
        .iter()
        .filter(|&i| ranges.iter().any(|range| range.contains(i)))
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
