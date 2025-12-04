use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split(',')
            .map(|s| {
                let (start, end) = s.split_once('-').unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .flat_map(|range| range.filter(is_invalid_id))
            .sum(),
    )
}

fn is_invalid_id(id: &u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return false;
    }
    let (first, second) = id_str.split_at(id_str.len() / 2);
    first == second
}

fn is_invalid_id_2(id: &u64) -> bool {
    let id_str = id.to_string();
    let chars = id_str.chars();
    (1..(id_str.len())).any(|n| {
        chars
            .clone()
            .chunks(n)
            .into_iter()
            .map(|c| c.collect_vec())
            .all_equal_value()
            .is_ok()
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split(',')
            .map(|s| {
                let (start, end) = s.split_once('-').unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .flat_map(|range| range.filter(is_invalid_id_2))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(11, 23, vec![11, 22])]
    #[case(95, 116, vec![99])]
    #[case(998, 1013, vec![1010])]
    #[case(1188511880, 1188511891, vec![1188511885])]
    #[case(222220, 222225, vec![222222])]
    #[case(1698522, 1698529, vec![])]
    #[case(446443, 446450, vec![446446])]
    #[case(38593856, 38593863, vec![38593859])]
    fn test_ranges(#[case] start: u64, #[case] end: u64, #[case] expected: Vec<u64>) {
        let invalid_ids: Vec<u64> = (start..=end).filter(is_invalid_id).collect();
        assert_eq!(invalid_ids, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
