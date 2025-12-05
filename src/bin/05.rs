use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
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
    (ranges, ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_input, _) = input.split_once("\n\n").unwrap();
    let ranges = ranges_input.lines().map(|l| {
        let (start, end) = l.split_once('-').unwrap().into();
        let start = start.parse::<usize>().expect("Could not parse number");
        let end = end.parse::<usize>().expect("Could not parse number");
        RangeInclusive::new(start, end)
    });

    let mut ranges_map: BTreeMap<usize, RangeInclusive<usize>> = BTreeMap::new();
    for mut range in ranges {
        for (_, other) in &ranges_map {
            // todo: use .range(&range)
            if let Some(subtracted_range) = range_difference(&range, &other) {
                range = subtracted_range;
            } else {
                // Range was fully contained within another range
                continue;
            }
        }
        ranges_map.insert(*range.start(), range.clone());
    }
    let sum: usize = ranges_map.into_iter().map(|(_, range)| range.count()).sum();

    Some(sum as u64)
}

fn range_difference(
    minuend: &RangeInclusive<usize>,
    subtrahend: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let start = minuend.start().max(subtrahend.start());
    let end = minuend.end().min(subtrahend.end());
    if start > end {
        0
    } else {
        end - start + 1
    }

    RangeInclusive
}

// fn count_overlap(a: RangeInclusive<usize>, b: &RangeInclusive<usize>) -> Option<RangeInclusive<usize>> {
//     let start = a.start().max(b.start());
//     let end = a.end().min(b.end());
//     if start > end { 0 } else { end - start + 1 }
// }

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
