use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    value: u32,
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Position {
    fn new() -> Position {
        Position { value: 0, x: 0, y: 0 }
    }
}

fn main() {
    let Ok(data) = fs::read_to_string("../_input.txt") else { return };

    let (mut start, mut end) = (Position::new(), Position::new());
    let mut moves: HashMap<Position, Vec<Move>> = HashMap::new();

    for (l, line) in data.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let char_code = ch as u32;
            match ch {
                'S' => {
                    start = Position {
                        value: 0,
                        x: c as u32,
                        y: l as u32,
                    };
                    moves.insert(start, vec![]);
                }
                'E' => {
                    end = Position {
                        value: 26, // Since 0-25 represents A-Z, 26 represents end
                        x: c as u32,
                        y: l as u32,
                    };
                    moves.insert(end, vec![]);
                }
                _ => {
                    moves.insert(
                        Position {
                            value: char_code - 97,
                            x: c as u32,
                            y: l as u32,
                        },
                        vec![],
                    );
                }
            }
        }
    }

    println!("Start: {:?} End: {:?} Rest: {:?}", start, end, moves);
    println!("Moves: {:?}", find_moves(&mut moves, start, None));
}

fn find_moves(moves: &mut HashMap<Position, Vec<Move>>, from: Position, last_dir: Option<Move>) -> Vec<Move> {
    println!(
        "From Position: {:?}, Direction: {:?}, Last Moves: {:?}",
        from,
        last_dir,
        moves.get(&from).unwrap()
    );

    if from.value == 26 {
        println!("Found end");
        moves.get_mut(&from).unwrap().push(last_dir.unwrap_or(Move::None));
        return moves.get(&from).unwrap().to_owned();
    }

    // Since we have scanned linearly, the items at bottom of matrix will be at higer inde, so for checking some index from above, we need to subtract 1 from y
    if from.y > 0 && last_dir != Some(Move::Up) {
        if moves.contains_key(&Position { y: from.y - 1, ..from }) {
            let above = Position { y: from.y - 1, ..from };
            println!("key found (above): {:?}", &above);
            let moves_found = &mut find_moves(moves, above, Some(Move::Down));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        } else if moves.contains_key(&Position {
            value: from.value + 1,
            y: from.y - 1,
            ..from
        }) {
            let above = Position {
                value: from.value + 1,
                y: from.y - 1,
                ..from
            };
            println!("key found (above + 1): {:?}", &above);
            let moves_found = &mut find_moves(moves, above, Some(Move::Down));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        }
    } else {
        println!("None for above at {:?}", from);
        // moves.as_mut().unwrap_or(&mut vec![]).push(Move::Up);
    }

    if last_dir != Some(Move::Right) {
        if moves.contains_key(&Position { x: from.x + 1, ..from }) {
            let right = Position { x: from.x + 1, ..from };
            println!("key found (right): {:?}", &right);
            let moves_found = &mut find_moves(moves, right, Some(Move::Left));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        } else if moves.contains_key(&Position {
            value: from.value + 1,
            x: from.x + 1,
            ..from
        }) {
            let right = Position {
                value: from.value + 1,
                x: from.x + 1,
                ..from
            };
            println!("key found (right + 1): {:?}", &right);
            let moves_found = &mut find_moves(moves, right, Some(Move::Left));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        }
    }

    if last_dir != Some(Move::Down) {
        if moves.contains_key(&Position { y: from.y + 1, ..from }) {
            let below = Position { y: from.y + 1, ..from };
            println!("key found (below): {:?}", &below);
            let moves_found = &mut find_moves(moves, below, Some(Move::Up));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        } else if moves.contains_key(&Position {
            value: from.value + 1,
            y: from.y + 1,
            ..from
        }) {
            let below = Position {
                value: from.value + 1,
                y: from.y + 1,
                ..from
            };
            println!("key found (below + 1): {:?}", &below);
            let moves_found = &mut find_moves(moves, below, Some(Move::Up));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        };
    }

    if from.x > 0 && last_dir != Some(Move::Left) {
        if moves.contains_key(&Position { x: from.x - 1, ..from }) {
            let left = Position { x: from.x - 1, ..from };
            println!("key found (left): {:?}", &left);
            let moves_found = &mut find_moves(moves, left, Some(Move::Right));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        } else if moves.contains_key(&Position {
            value: from.value + 1,
            x: from.x - 1,
            ..from
        }) {
            let left = Position {
                value: from.value + 1,
                x: from.x - 1,
                ..from
            };
            println!("key found (left + 1): {:?}", &left);
            let moves_found = &mut find_moves(moves, left, Some(Move::Right));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Moves found: {:?}", moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());
        }
    } else {
        println!("None for left at {:?}", from);
        // moves.as_mut().unwrap_or(&mut vec![]).push(Move::Left);
    }

    moves.get_mut(&from).expect("error at last").to_vec()
}
