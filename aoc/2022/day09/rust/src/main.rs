use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
}

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else { return };

    let mut visited = HashSet::new();
    let mut rope = Rope {
        head: Position { x: 0, y: 0 },
        tail: Position { x: 0, y: 0 },
    };

    data.lines().for_each(|line| {
        let mut input = line.split_whitespace();
        let (direction, distance) = (
            input.next().expect("unreachable index"),
            input.next().expect("unreachable index").parse::<u32>().expect("NaN"),
            // Part1 took more than 3 days alone, only to find that I wasn't parsing the full string but taking chars at 1st & 3rd index
        );

        // println!("--> {direction}:{distance}");

        match direction {
            "U" => {
                for _ in 0..distance {
                    rope.head.x += 1;
                    if !is_adjacent(&rope) {
                        rope.tail.x = rope.head.x - 1;
                        rope.tail.y = rope.head.y;
                    };
                    // println!(
                    //     "--> U: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            "R" => {
                for _ in 0..distance {
                    rope.head.y += 1;
                    if !is_adjacent(&rope) {
                        rope.tail.y = rope.head.y - 1;
                        rope.tail.x = rope.head.x;
                    }
                    // println!(
                    //     "--> R: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            "D" => {
                for _ in 0..distance {
                    rope.head.x -= 1;
                    if !is_adjacent(&rope) {
                        rope.tail.x = rope.head.x + 1;
                        rope.tail.y = rope.head.y;
                    }
                    // println!(
                    //     "--> D: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            "L" => {
                for _ in 0..distance {
                    rope.head.y -= 1;
                    if !is_adjacent(&rope) {
                        rope.tail.y = rope.head.y + 1;
                        rope.tail.x = rope.head.x;
                    }
                    // println!(
                    //     "--> L: h.x={} h.y={} t.x={} t.y={}",
                    //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
                    // );
                    visited.insert(rope.tail.clone());
                }
            }
            _ => {
                todo!();
            }
        }
    });

    // _draw(&visited);

    println!("part_01: {:?}", visited.len());
}

fn is_adjacent(rope: &Rope) -> bool {
    let (hx, hy, tx, ty) = (rope.head.x, rope.head.y, rope.tail.x, rope.tail.y);

    // Overlap
    hx == tx && hy == ty
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
        || hx == tx + 1 && hy == ty + 1
}

fn _draw(visited: &HashSet<Position>) {
    // These values are here from trial & error, though they can be found using some kinda accumulator in the primary function
    // The addition in x & y pos is to cover for negative values since array indexes can't be negative

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
