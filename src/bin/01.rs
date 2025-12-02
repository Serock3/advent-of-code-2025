use num::Integer;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        let mut change: i32 = line[1..].parse().unwrap();
        let dir = line.chars().next().unwrap();
        if dir == 'L' {
            change *= -1;
        }
        dial += change;
        dial %= 100;
        if dial == 0 {
            zero_count += 1;
        };
    }
    Some(zero_count)
}

fn inc_count(dial: &mut i32, zero_count: &mut u64, mut change: i32, dir: char) {
    if dir == 'L' {
        change *= -1;
    }
    *dial += change;
    *dial %= 100;
    if *dial == 0 {
        *zero_count += 1;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut zero_count: u64 = 0;
    for line in input.lines() {
        let change: i32 = line[1..].parse().unwrap();
        let dir = line.chars().next().unwrap();
        for _ in 0..change {
            inc_count(&mut dial, &mut zero_count, 1, dir);
        }
        // if line.chars().next().unwrap() == 'L' {
        //     change *= -1;
        // }

        // dial += dbg!(change);
        // if dial == 0 {
        //     zero_count -= 1;
        // }
        // let turns = turns(dbg!(dial));
        // println!("turn: {turns:?}");
        // zero_count += turns;
        // // zero_count += dbg!(dial.div_rem_euclid(&100).0).unsigned_abs() as u64;
        // dial = dial.mod_floor(&100);
    }
    Some(zero_count)
}

fn turns(dial: i32) -> u64 {
    if dial == 0 {
        1
    } else {
        dial.div_mod_floor(&100).0.abs() as u64
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(
            &"L51
L1",
        );
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two(
            &"L51
L200",
        );
        assert_eq!(result, Some(3));
    }

    use rstest::rstest;

    #[rstest]
    #[case(-5, 1)]
    #[case(-105, 2)]
    #[case(-205, 3)]
    #[case(5, 0)]
    #[case(105, 1)]
    #[case(505, 5)]
    #[case(0, 1)]
    fn test_neg_quotient(#[case] input: i32, #[case] expected: u64) {
        assert_eq!(expected, turns(input))
    }
}
