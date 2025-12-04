use advent_of_code::{
    Pos, get_adjacent_positions, get_surrounding_positions, parse_char_matrix, print_matrix,
};
use ndarray::Array2;
use ndarray_ndimage::{PadMode, pad};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let matrix = parse_char_matrix(input);

    let removable_balls = matrix
        .indexed_iter()
        .filter(|(_, c)| **c == '@')
        .filter(|(pos, _)| {
            let num_adjacent = get_surrounding_positions(*pos, matrix.dim())
                .filter(|pos| matrix[(pos.0, pos.1)] == '@')
                .count();
            println!("{:?}: {}", pos, num_adjacent);
            num_adjacent < 4
        });

    Some(removable_balls.count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut matrix = parse_char_matrix(input);

    let mut removed_balls = 0;
    loop {
        let removable_balls = matrix
            .indexed_iter()
            .map(|(pos, &char)| (pos, char))
            .filter(|(_, c)| *c == '@')
            .filter(|(pos, _)| {
                let num_adjacent = get_surrounding_positions(*pos, matrix.dim())
                    .filter(|pos| matrix[(pos.0, pos.1)] == '@')
                    .count();
                println!("{:?}: {}", pos, num_adjacent);
                num_adjacent < 4
            })
            .collect::<Vec<_>>();
        if removable_balls.is_empty() {
            break;
        }
        removed_balls += removable_balls.len();
        for (ball_pos, prev_char) in removable_balls {
            println!("Removing '{}' at {:?}", prev_char, ball_pos);
            matrix[ball_pos] = '.';
        }
    }

    Some(removed_balls as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
