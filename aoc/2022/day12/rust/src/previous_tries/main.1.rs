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

    {
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
                        moves.insert(
                            Position {
                                value: 0,
                                x: c as u32,
                                y: l as u32,
                            },
                            vec![],
                        );
                    }
                    'E' => {
                        end = Position {
                            value: 26,
                            x: c as u32,
                            y: l as u32,
                        };
                        moves.insert(
                            Position {
                                value: 26,
                                x: c as u32,
                                y: l as u32,
                            },
                            vec![],
                        );
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
    }

    println!("Start: {:?} End: {:?} Rest: {:?}", start, end, moves);
    println!("Moves: {:?}", _find_moves(&mut moves, start, None));
}

fn _find_moves(moves: &mut HashMap<Position, Vec<Move>>, from: Position, last_dir: Option<Move>) -> Vec<Move> {
    println!(
        "From Position: {:?}, Direction: {:?}, Last Moves: {:?}",
        from,
        last_dir,
        moves.get(&from).unwrap()
    );

    // let Some(moves) = moves.get_mut(&from) else { return vec![] };

    if from.value == 26 {
        println!("Found end");
        moves.to_owned().get_mut(&from).unwrap().push(Move::None);
        return moves.get(&from).unwrap().to_owned();
        // return moves.get(&from).expect(&format!("Failed to get key for {:?}", &from)).to_owned();
    }

    // Since we have scanned linearly, the items at bottom of matrix will be at higer inde, so for checking some index from above, we need to subtract 1 from y
    if from.y > 0 && last_dir != Some(Move::Up) {
        if moves.contains_key(&Position { y: from.y - 1, ..from }) {
            let above = Position { y: from.y - 1, ..from };
            println!("key found (above): {:?}", &above);
            let moves_found = &mut _find_moves(&mut moves.clone(), above, Some(Move::Down));
            moves.to_owned().get_mut(&from).unwrap().append(moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /* if let Some(_) = {
                println!("Moving above");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Up(Some(above)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, above);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Up);
            } */
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
            let moves_found = &mut _find_moves(&mut moves.clone(), above, Some(Move::Down));
            moves.get_mut(&from).unwrap().append(moves_found);
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /* if let Some(_) = _find_moves(moves, above, Some(Move::Down)) {
                println!("Moving above");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Up(Some(above)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, above);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Up);
            } */
        }
    } else {
        println!("None for above at {:?}", from);
        // moves.as_mut().unwrap_or(&mut vec![]).push(Move::Up);
    }

    if last_dir != Some(Move::Right) {
        if moves.contains_key(&Position { x: from.x + 1, ..from }) {
            let right = Position { x: from.x + 1, ..from };
            println!("key found (right): {:?}", &right);
            moves.to_owned().get_mut(&from).unwrap().append(&mut _find_moves(&mut moves.clone(), right, Some(Move::Left)));
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            // if let Some(_) = _find_moves(moves, right, Some(Move::Left)) {
            //     println!("Moving right");
            //     moves.as_mut().unwrap_or(&mut vec![]).push(Move::Right(Some(right)));
            //     println!("{:?}", moves);
            // } else {
            //     println!("Stopping for {:?} at {:?}", from, right);
            //     moves.as_mut().unwrap_or(&mut vec![]).push(Move::Right);
            // }
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
            moves.to_owned().get_mut(&from).unwrap().append(&mut _find_moves(&mut moves.clone(), right, Some(Move::Left)));
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /*  if let Some(_) = _find_moves(moves, right, Some(Move::Left)) {
                println!("Moving right");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Right(Some(right)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, right);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Right);
            } */
        }
    }

    if last_dir != Some(Move::Down) {
        if moves.contains_key(&Position { y: from.y + 1, ..from }) {
            let below = Position { y: from.y + 1, ..from };
            println!("key found (below): {:?}", &below);
            moves.to_owned().get_mut(&from).unwrap().append(&mut _find_moves(&mut moves.clone(), below, Some(Move::Up)));
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /* if let Some(_) = _find_moves(moves, below, Some(Move::Up)) {
                println!("Moving below");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Down(Some(below)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, below);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Down);
            } */
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
            moves.to_owned().get_mut(&from).unwrap().append(&mut _find_moves(&mut moves.clone(), below, Some(Move::Up)));
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /* if let Some(_) = _find_moves(moves, below, Some(Move::Up)) {
                println!("Moving below");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Down(Some(below)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, below);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Down);
            } */
        };
    }

    if from.x > 0 && last_dir != Some(Move::Left) {
        if moves.contains_key(&Position { x: from.x - 1, ..from }) {
            let left = Position { x: from.x - 1, ..from };
            println!("key found (left): {:?}", &left);
            moves.to_owned().get_mut(&from).unwrap().append(&mut _find_moves(&mut moves.clone(), left, Some(Move::Right)));
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /* if let Some(_) = _find_moves(moves, left, Some(Move::Right)) {
                println!("Moving left");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Left(Some(left)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, left);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Left);
            } */
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
            moves.to_owned().get_mut(&from).unwrap().append(&mut _find_moves(&mut moves.clone(), left, Some(Move::Right)));
            println!("Updated at location: {:?}, Moves: {:?}", &from, moves.get(&from).unwrap());

            /* if let Some(_) = _find_moves(moves, left, Some(Move::Right)) {
                println!("Moving left");
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Left(Some(left)));
                println!("{:?}", moves);
            } else {
                println!("Stopping for {:?} at {:?}", from, left);
                moves.as_mut().unwrap_or(&mut vec![]).push(Move::Left);
            } */
        }
    } else {
        println!("None for left at {:?}", from);
        // moves.as_mut().unwrap_or(&mut vec![]).push(Move::Left);
    }

    moves.get(&from).expect("error at last").to_vec()
}
