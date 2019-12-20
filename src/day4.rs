#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<u32> {
    input.split('-').map(|num| num.parse().unwrap()).collect()
}

fn is_valid_password_part1(mut num: u32) -> bool {
    let mut repeating = false;
    let mut last = num % 10;
    num /= 10;
    while num > 0 {
        let current = num % 10;
        match current {
            c if c == last => repeating = true,
            c if c > last => return false,
            _ => (),
        }
        last = current;
        num /= 10;
    }
    repeating
}

fn is_valid_password_part2(mut num: u32) -> bool {
    let mut repeating = false;
    let mut group_has_two_repeat = false;
    let mut has_two_repeat = false;
    let mut last = num % 10;
    let mut repeating_num = last;
    num /= 10;
    while num > 0 {
        let current = num % 10;
        match current {
            c if c == last => {
                if !repeating {
                    repeating = true;
                    group_has_two_repeat = true;
                    repeating_num = c;
                } else if repeating && c == repeating_num {
                    group_has_two_repeat = false;
                } else {
                    repeating = false;
                }
            }
            c if c > last => return false,
            _ => {
                if group_has_two_repeat {
                    has_two_repeat = true
                }
                repeating = false;
            }
        }
        last = current;
        num /= 10;
    }
    has_two_repeat || group_has_two_repeat
}

#[aoc(day4, part1)]
fn part1(input_range: &[u32]) -> usize {
    let start = input_range[0];
    let end = input_range[1];

    (start..end).fold(0usize, |sum, num| {
        sum + is_valid_password_part1(num) as usize
    })
}

#[aoc(day4, part2)]
fn part2(input_range: &[u32]) -> usize {
    let start = input_range[0];
    let end = input_range[1];

    (start..end).fold(0usize, |sum, num| {
        sum + is_valid_password_part2(num) as usize
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_0() {
        assert_eq!(is_valid_password_part1(111_111), true);
    }

    #[test]
    fn part1_example_1() {
        assert_eq!(is_valid_password_part1(223_450), false);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(is_valid_password_part1(123_789), false);
    }

    #[test]
    fn part2_example_0() {
        assert_eq!(is_valid_password_part2(112_233), true);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(is_valid_password_part2(123_444), false);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(is_valid_password_part2(111_122), true);
    }

    #[test]
    fn part2_home_backed_example_0() {
        assert_eq!(is_valid_password_part2(111_111), false);
    }

    #[test]
    fn part2_home_backed_example_1() {
        assert_eq!(is_valid_password_part2(123_456), false);
    }

    #[test]
    fn part2_home_backed_example_2() {
        assert_eq!(is_valid_password_part2(123_354), false);
    }

    #[test]
    fn part2_home_backed_example_3() {
        assert_eq!(is_valid_password_part2(123_345), true);
    }

    #[test]
    fn part2_home_backed_example_4() {
        assert_eq!(is_valid_password_part2(123_555), false);
    }

    #[test]
    fn part2_home_backed_example_5() {
        assert_eq!(is_valid_password_part2(223_555), true);
    }
}
