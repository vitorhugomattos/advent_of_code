use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("reading input file");

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input.clone()));
}

pub fn part_1(input: String) -> isize {
    input.trim().chars().fold(0, |acc, x| match x {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => unreachable!(),
    })
}

pub fn part_2(input: String) -> isize {
    let mut floor = 0;

    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!()
        }

        if floor == -1 {
            return i as isize + 1
        }
    }

    floor
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part_1() {
        let test_inputs = HashMap::from([
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ]);

        for (input, output) in test_inputs.into_iter() {
            assert_eq!(part_1(input.to_string()), output);
        }
    }

    #[test]
    fn test_part_2() {
        let test_inputs = HashMap::from([(")", 1), ("()())", 5)]);

        for (input, output) in test_inputs.into_iter() {
            assert_eq!(part_2(input.to_string()), output);
        }
    }
}
