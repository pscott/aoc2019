pub fn compute_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|num| num.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part_one(modules: &[u64]) -> u64 {
    modules.iter().map(|i| compute_fuel(*i)).sum()
}

#[aoc(day1, part2)]
pub fn toto(num: &[u64]) -> u64 {
	num[0] + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(compute_fuel(0), 0);
    }

    #[test]
    fn twelve() {
        assert_eq!(compute_fuel(12), 2);
    }

    #[test]
    fn big() {
        assert_eq!(compute_fuel(100_756), 33583);
    }

    #[test]
    fn total_big() {
        let modules = vec![12, 0, 100_756];
        assert_eq!(part_one(&modules), 33585);
    }
}
