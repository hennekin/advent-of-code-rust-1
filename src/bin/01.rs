advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut count: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let dir = line.chars().next().unwrap();
        let distance: i64 = line[1..].parse().unwrap();

        if dir == 'L' {
            position = (position + 100 - distance) % 100;
        } else {
            position = (position + distance) % 100;
        }

        if position == 0 {
            count += 1;
        }
    }

    Some(count as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut count: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let dir = line.chars().next().unwrap();
        let distance: i64 = line[1..].parse().unwrap();

        if dir == 'L' {
            position = (position + 100 - distance) % 100;
        } else {
            position = (position + distance) % 100;
        }

        if position == 0 {
            count += 1;
        }
    }

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input1 = "R50";

        assert_eq!(part_one(input1), Some(1));

        let input2 = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_one(input2), Some(3));

        let input_file = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input_file), Some(1152));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
