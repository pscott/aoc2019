use aoc_runner_derive::{aoc, aoc_generator};

fn compute_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|num| num.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(modules: &[u64]) -> u64 {
    modules.iter().fold(0u64, |sum, i| sum + compute_fuel(*i))
}

fn total_fuel(mut mass: u64) -> u64 {
    let mut total_fuel = 0;
    while mass > 0 {
        let fuel = compute_fuel(mass);
        total_fuel += fuel;
        mass = fuel;
    }
    total_fuel
}

#[aoc(day1, part2)]
fn part2(modules: &[u64]) -> u64 {
    modules.iter().fold(0u64, |sum, i| sum + total_fuel(*i))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn zero() {
//         assert_eq!(compute_fuel(0), 0);
//     }

//     #[test]
//     fn twelve() {
//         assert_eq!(compute_fuel(12), 2);
//     }

//     #[test]
//     fn big() {
//         assert_eq!(compute_fuel(100_756), 33583);
//     }

//     #[test]
//     fn total_big() {
//         let modules = vec![12, 0, 100_756];
//         assert_eq!(part_one(&modules), 33585);
//     }
// }
