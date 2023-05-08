use std::{collections::HashSet, fs, i32};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Position {
    fn move_by(&mut self, position: &Self) {
        self.x += position.x;
        self.y += position.y;
    }
}

fn part_01(lines: Vec<(Direction, u32)>) -> usize {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Rope {
        head: Position,
        tail: Position,
    }

    let mut visited = HashSet::new();
    let mut rope = Rope {
        head: Position { x: 0, y: 0 },
        tail: Position { x: 0, y: 0 },
    };

    lines.into_iter().for_each(|(direction, distance)| {
        match direction {
            Direction::Top => {
                for _ in 0..distance {
                    rope.head.y += 1;
                    if let Some(pos) = maybe_move_to(&rope.head, &rope.tail) {
                        rope.tail.move_by(&pos);
                    };
                    // println!(
                    //     "--> U: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            Direction::Right => {
                for _ in 0..distance {
                    rope.head.x += 1;
                    if let Some(pos) = maybe_move_to(&rope.head, &rope.tail) {
                        rope.tail.move_by(&pos);
                    };
                    // println!(
                    //     "--> R: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            Direction::Bottom => {
                for _ in 0..distance {
                    rope.head.y -= 1;
                    if let Some(pos) = maybe_move_to(&rope.head, &rope.tail) {
                        rope.tail.move_by(&pos);
                    };
                    // println!(
                    //     "--> D: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            Direction::Left => {
                for _ in 0..distance {
                    rope.head.x -= 1;
                    if let Some(pos) = maybe_move_to(&rope.head, &rope.tail) {
                        rope.tail.move_by(&pos);
                    };
                    // println!(
                    //     "--> L: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
        }
    });

    // _draw(&visited);

    visited.len()
}

fn part_02(lines: Vec<(Direction, u32)>) -> usize {
    let mut rope = vec![Position { x: 0, y: 0 }; 10];
    let mut visited = HashSet::new();

    lines.into_iter().for_each(|(direction, distance)| match direction {
        Direction::Top => {
            for _ in 0..distance {
                for index in 0..9 {
                    if index == 0 {
                        rope[index].y += 1;
                    };
                    if let Some(pos) = maybe_move_to(&rope[index], &rope[index + 1]) {
                        rope[index + 1].move_by(&pos);
                    };
                    visited.insert(rope[9]);
                }
                // println!("UP: {:?}", rope);
            }
        }
        Direction::Right => {
            for _ in 0..distance {
                for index in 0..9 {
                    if index == 0 {
                        rope[index].x += 1;
                    };
                    if let Some(pos) = maybe_move_to(&rope[index], &rope[index + 1]) {
                        rope[index + 1].move_by(&pos);
                    };
                    visited.insert(rope[9]);
                }
                // println!("RIGHT: {:?}", rope);
            }
        }
        Direction::Bottom => {
            for _ in 0..distance {
                for index in 0..9 {
                    if index == 0 {
                        rope[index].y -= 1;
                    };
                    if let Some(pos) = maybe_move_to(&rope[index], &rope[index + 1]) {
                        rope[index + 1].move_by(&pos);
                    };
                    visited.insert(rope[9]);
                }
                // println!("DOWN: {:?}", rope);
            }
        }
        Direction::Left => {
            for _ in 0..distance {
                for index in 0..9 {
                    if index == 0 {
                        rope[index].x -= 1;
                    };
                    if let Some(pos) = maybe_move_to(&rope[index], &rope[index + 1]) {
                        rope[index + 1].move_by(&pos);
                    };
                    visited.insert(rope[9]);
                }
                // println!("LEFT: {:?}", rope);
            }
        }
    });

    visited.len()
}

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else { return };

    let lines = data
        .lines()
        .map(|line| {
            let mut input = line.split_whitespace();
            (
                match input.next() {
                    Some("U") => Direction::Top,
                    Some("R") => Direction::Right,
                    Some("D") => Direction::Bottom,
                    Some("L") => Direction::Left,
                    Some(&_) | None => panic!("Invalid direction"),
                },
                input.next().expect("unreachable index").parse::<u32>().expect("NaN"),
            )
        })
        .collect::<Vec<(Direction, u32)>>();

    println!("part_01: {:?}", part_01(lines.clone()));
    println!("part_02: {:?}", part_02(lines.clone()));
}

fn maybe_move_to(curr_pos: &Position, next_pos: &Position) -> Option<Position> {
    let (diff_x, diff_y) = (curr_pos.x - next_pos.x, curr_pos.y - next_pos.y);

    // Using algo from https://www.youtube.com/watch?v=QfSPVrWKGcU
    if i32::abs(diff_x) > 1 || i32::abs(diff_y) > 1 {
        Some(Position {
            x: if diff_x != 0 {
                diff_x / (i32::abs(diff_x))
            } else {
                0
            },
            y: if diff_y != 0 {
                diff_y / (i32::abs(diff_y))
            } else {
                0
            },
        })
    } else {
        None
    }

    // Below approach works well for immediate next positions, but fails for positions farther than 1 unit

    /* hx == tx && hy == ty
    // Above
    || hx == tx - 1 && hy == ty - 1
    || hx == tx - 1 && hy == ty
    || hx == tx - 1 && hy == ty + 1
    // Middle
    || hx == tx && hy == ty - 1
    || hx == tx && hy == ty + 1
    // Below
    || hx == tx + 1 && hy == ty - 1
    || hx == tx + 1 && hy == ty
    || hx == tx + 1 && hy == ty + 1 */
}

fn _draw(visited: &HashSet<Position>) {
    // These values are here from trial & error, though they can be found using some kinda accumulator in the primary function
    // The addition in x & y pos is to cover for negative values since array indexes can't be negative

    // This doesn't work anymore for some reason, will need to look how to draw these kind of terminal graphics

    let mut grid = vec![vec!['.'; 281]; 260];

    visited.iter().for_each(|pos| {
        grid[(pos.x + 29) as usize][(pos.y + 46) as usize] =
            if pos.x == 0 && pos.y == 0 { 'S' } else { '#' };
    });

    grid.reverse();

    grid.iter().for_each(|row| {
        row.iter().for_each(|col| print!("{col}"));
        println!();
    });
}
