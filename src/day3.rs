use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone)]
struct Movement {
    dir: Direction,
    num: u16,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Movement {
    fn from(input: &str) -> Self {
        let num = input[1..].parse().unwrap();
        match input.chars().next().unwrap() {
            'R' => Self {
                dir: Direction::Right,
                num,
            },
            'L' => Self {
                dir: Direction::Left,
                num,
            },
            'U' => Self {
                dir: Direction::Up,
                num,
            },
            'D' => Self {
                dir: Direction::Down,
                num,
            },
            e => panic!("error while parsing for direction: got {}", e),
        }
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<Movement>> {
    input
        .lines()
        .map(|line| line.split(',').map(Movement::from).collect())
        .collect()
}

fn compute_distance(point: Point) -> u64 {
    point.x.abs() as u64 + point.y.abs() as u64
}

fn compute_pos(start: Point, movement: Direction) -> Point {
    let (x_move, y_move) = match movement {
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
    };
    Point::new(start.x + x_move, start.y + y_move)
}

fn store_path(vec: &[Movement]) -> HashMap<Point, u64> {
    let mut mapping = HashMap::new();
    let mut current = Point::new(0, 0);

    for movement in vec {
        for _ in 0..movement.num {
            current = compute_pos(current, movement.dir);
            let distance = compute_distance(current);
            mapping.insert(current, distance);
        }
    }
    mapping
}

fn store_steps(vec: &[Movement]) -> HashMap<Point, u64> {
    let mut mapping = HashMap::new();
    let mut current = Point::new(0, 0);
    let mut distance = 0;

    for movement in vec {
        for _ in 0..movement.num {
            current = compute_pos(current, movement.dir);
            distance += 1;
            mapping.insert(current, distance);
        }
    }
    mapping
}

#[aoc(day3, part1)]
fn part1(lines: &[Vec<Movement>]) -> u64 {
    let mut first_wire = store_path(&lines[0]);
    let second_wire = store_path(&lines[1]);

    first_wire.retain(|k, _| second_wire.contains_key(k));
    *first_wire.values().min().unwrap()
}

#[aoc(day3, part2)]
fn part2(lines: &[Vec<Movement>]) -> u64 {
    let mut first_wire = store_steps(&lines[0]);
    let second_wire = store_steps(&lines[1]);
    let mut steps = vec![];

    first_wire.retain(|k, _| second_wire.contains_key(k));
    for (k, v) in first_wire {
        steps.push(v + second_wire.get(&k).unwrap());
    }
    *steps.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let input = "R1,D1,U1,L1\nL1,R1";

        assert_eq!(part1(&input_generator(&input)), 0);
    }

    #[test]
    fn part1_example_0() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";

        assert_eq!(part1(&input_generator(&input)), 6);
    }

    #[test]
    fn part1_example_1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part1(&input_generator(&input)), 159);
    }

    #[test]
    fn part1_example_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part1(&input_generator(&input)), 135);
    }

    #[test]
    fn part2_example_0() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";

        assert_eq!(part2(&input_generator(&input)), 30);
    }

    #[test]
    fn part2_example_1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part2(&input_generator(&input)), 610);
    }

    #[test]
    fn part2_example_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part2(&input_generator(&input)), 410);
    }
}
