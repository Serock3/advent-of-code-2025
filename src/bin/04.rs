use advent_of_code::{
    Pos, get_adjacent_positions, get_surrounding_positions, parse_char_matrix, print_matrix,
};
use ndarray::Array2;
use ndarray_ndimage::{PadMode, pad};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let matrix = parse_char_matrix(input);

    let res = matrix
        .indexed_iter()
        .filter(|(_, c)| **c == '@')
        .filter(|(pos, _)| {
            let num_adjacent = get_surrounding_positions(*pos, matrix.dim())
                .filter(|pos| matrix[(pos.0, pos.1)] == '@')
                .count();
            println!("{:?}: {}", pos, num_adjacent);
            num_adjacent < 4
        })
        .count();

    // let padding_config = vec![(1, 1), (1, 1)];

    // for window in pad(&matrix, &padding_config, PadMode::Constant(0)).windows((3, 3)) {
    //     println!("");
    //     // print_matrix(&window);
    //     //
    //     let num_paper = window
    //         .flatten()
    //         .iter()
    //         .enumerate()
    //         .filter(|(i, el)| **el == '@' && *i % 5 != 0)
    //         .count();
    //     println!("{:?}", num_paper);
    // }
    Some(res as u64)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
