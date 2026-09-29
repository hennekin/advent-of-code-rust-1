advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    struct LockWheel {
        number: i32,
        count: i32
    }
    impl LockWheel {

        fn new() -> Self {
            Self { number: 50, count: 0}
        }

        pub fn turn_left(&mut self, value: i32) {
            self.number = (self.number - value).rem_euclid(100);

            if self.number == 0 {
                self.count += 1;
            }
        }

        pub fn turn_right(&mut self, value: i32) {
            /*
             * The dial has positions 0 through 99. `rem_euclid(100)` wraps
             * values above 99 back to 0 and keeps the result non-negative,
             * so values below 0 wrap around to 99.
             */
            self.number = (self.number + value).rem_euclid(100);

            if self.number == 0 {
                self.count += 1;
            }
        }
    }

    let mut wheel = LockWheel::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let (direction, distance) = line.split_at(1);
        let value = distance.parse::<i32>().ok()?;

        match direction {
            "L" => wheel.turn_left(value),
            "R" => wheel.turn_right(value),
            _ => return None,
        }
    }

    Some(wheel.count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
