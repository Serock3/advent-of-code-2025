use itertools::Itertools;
use std::{
    collections::BTreeMap,
    ops::{Range, RangeInclusive},
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input);

    let count = ingredients
        .iter()
        .filter(|&i| ranges.iter().any(|range| range.contains(i)))
        .count();

    Some(count as u64)
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<usize>> = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            let start = start.parse::<usize>().expect("Could not parse number");
            let end = end.parse::<usize>().expect("Could not parse number");
            RangeInclusive::new(start, end)
        })
        .collect_vec();

    let ingredients: Vec<usize> = ingredients
        .lines()
        .map(|i| i.parse::<usize>().expect("Could not parse number"))
        .collect();
    (ranges, ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_input, _) = input.split_once("\n\n").unwrap();
    let ranges = ranges_input.lines().map(|l| {
        let (start, end) = l.split_once('-').unwrap();
        let start = start.parse::<usize>().expect("Could not parse number");
        let end = end.parse::<usize>().expect("Could not parse number");
        Range {
            start,
            end: end + 1,
        }
    });

    let mut ranges_map: BTreeMap<usize, Range<usize>> = BTreeMap::new();
    for mut range in ranges {
        for other in ranges_map.values_mut() {
            remove_overlap(&mut range, other);
        }
        ranges_map.insert(range.start, range.clone());
    }
    let sum: usize = ranges_map
        .clone()
        .into_values()
        .map(|range| range.count())
        .sum();

    Some(sum as u64)
}

fn remove_overlap(range_a: &mut Range<usize>, range_b: &mut Range<usize>) {
    let (first_range, second_range) = if range_a.start < range_b.start {
        (range_a, range_b)
    } else {
        (range_b, range_a)
    };
    // |---|
    //    |---|
    if first_range.end > second_range.start {
        second_range.start = first_range.end;
    }
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
        assert_eq!(result, Some(14));
    }
}
