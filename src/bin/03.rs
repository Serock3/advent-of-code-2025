use advent_of_code::parse_int_matrix;
use itertools::Itertools;
use ndarray::{Axis, Slice, s};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let matrix = parse_int_matrix::<u64>(input);

    Some(
        matrix
            .rows()
            .into_iter()
            .map(|mut row| {
                let first_digit_idx_flipped =
                    row.slice(s![0..-1]).iter().rev().position_max().unwrap();
                let first_digit_idx = row.len() - first_digit_idx_flipped - 2;
                let first_digit = row[first_digit_idx];
                row.slice_axis_inplace(Axis(0), Slice::from(first_digit_idx + 1..));
                let second_digit = row.into_iter().max().unwrap();
                first_digit * 10 + second_digit
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let matrix = parse_int_matrix::<u8>(input);

    Some(
        matrix
            .rows()
            .into_iter()
            .map(|mut row| {
                (0..12)
                    .map(|n| {
                        let available_digits = row.slice(s![0..(row.len() + n - 11)]);
                        let digit_idx_flipped =
                            available_digits.iter().rev().position_max().unwrap();
                        let digit_idx = available_digits.len() - digit_idx_flipped - 1;
                        let digit = available_digits[digit_idx];
                        row.slice_axis_inplace(Axis(0), Slice::from(digit_idx + 1..));
                        10u64.pow((11 - n) as u32) * digit as u64
                    })
                    .sum::<u64>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
