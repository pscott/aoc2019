const GOAL: u64 = 19_690_720;

#[aoc_generator(day2)]
fn generate_input(input: &str) -> Vec<u64> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn before_hand(program: &mut [u64], noun: u64, verb: u64) {
    program[1] = noun;
    program[2] = verb;
}

#[aoc(day2, part1)]
fn part1(program: &[u64]) -> u64 {
    // Cannot use ref_program: &mut [u64] due to aoc's macro
    let mut program = program.to_vec();

    before_hand(&mut program, 12, 2);
    computer(&mut program)
}

fn computer(program: &mut [u64]) -> u64 {
    let mut i = 0;

    loop {
        let op = program[i];

        if op == 99 {
            return program[0];
        }

        let left_index = program[i + 1];
        let right_index = program[i + 2];
        let left = *program.get(left_index as usize).expect("left");
        let right = *program.get(right_index as usize).expect("right");

        let target_index = program[i + 3];
        let target = program.get_mut(target_index as usize).expect("target");
        *target = match op {
            1 => left + right,
            2 => left * right,
            _ => 0,
        };
        i += 4;
    }
}

#[aoc(day2, part2)]
fn brute_force(init_state: &[u64]) -> u64 {
    let mut program = init_state.to_vec();

    for i in 0..99 {
        for j in 0..99 {
            let mut program = init_state.to_vec();
            before_hand(&mut program, i, j);
            if computer(&mut program) == GOAL {
                return i * 100 + j;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut input = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(computer(&mut input), 3500);
    }
}
