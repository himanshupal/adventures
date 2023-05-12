use core::panic;
use regex::{Captures, Regex};
use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    name: u64,
    test_against: u64,
    throw_to: (u64, u64),
    op: (String, String),
    inspected_count: u64,
    worry_levels: Vec<u64>,
}

fn solve(mut monkeys: Vec<Monkey>, is_part2: bool) -> u64 {
    let limit = if is_part2 { 10000 } else { 20 };

    let modulus: u64 = if is_part2 {
        monkeys.iter().map(|m| m.test_against).product()
    } else {
        0
    };

    for x in 1..=limit {
        for i in 0..monkeys.len() {
            let monkey = (monkeys[i]).to_owned();
            // println!("Monkey: {:?}", monkey);

            for value in monkey.worry_levels {
                let operand: u64;

                if let Ok(acc) = monkey.op.1.parse() {
                    operand = acc;
                } else {
                    operand = value.clone();
                };

                let mut worry = if monkey.op.0 == "*" {
                    value * operand
                } else if monkey.op.0 == "+" {
                    value + operand
                } else {
                    panic!("Unknown operator")
                };

                if is_part2 {
                    worry = worry % modulus;
                } else {
                    worry = worry / 3;
                }

                // println!("Worry: {worry}");

                if worry.clone() % monkey.test_against == 0 {
                    // println!("Throwing to: {}", monkey.throw_to.0);
                    monkeys[monkey.throw_to.0 as usize].worry_levels.push(worry);
                    monkeys[monkey.throw_to.0 as usize].inspected_count += 1;
                } else {
                    // println!("Throwing to: {}", monkey.throw_to.1);
                    monkeys[monkey.throw_to.1 as usize].worry_levels.push(worry);
                    monkeys[monkey.throw_to.1 as usize].inspected_count += 1;
                }

                monkeys[i].worry_levels.pop();
            }
        }

        // The items monkeys currently have in their hands & not inspected yet
        if x == limit {
            monkeys.iter_mut().for_each(|monkey| {
                monkey.inspected_count -= monkey.worry_levels.len() as u64;
            })
        }
    }

    let inspected_counts = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .collect::<Vec<u64>>();

    // println!("Inspected counts: {:?}", inspected_counts);

    let (max, next_max) = largest_two(inspected_counts);

    max * next_max
}

fn main() {
    let Ok(monkeys) = get_parsed_data("../input.txt") else { return };

    println!("part_01: {:?}", solve(monkeys.clone(), false));
    println!("part_02: {:?}", solve(monkeys.clone(), true));
}

fn get_parsed_data(file_path: &str) -> Result<Vec<Monkey>, ()> {
    let Ok(data) = fs::read_to_string(file_path) else { return Err(()) };

    let mut monkeys = vec![];

    data.lines().enumerate().step_by(7).for_each(|(line, _)| {
        let mut monkey = Monkey {
            name: 0,
            test_against: 1,
            throw_to: (0, 0),
            inspected_count: 0,
            worry_levels: vec![],
            op: (String::new(), String::new()),
        };

        for l in line..=line + 6 {
            if l == line + 6 {
                monkeys.push(monkey.clone());
                continue;
            };

            Regex::new(r"\d+|\+|\*|(\d+|old)$")
                .expect("Couldn't parse regex")
                .captures_iter(
                    data.lines()
                        .nth(l)
                        .expect(&format!("Couldn't extract line {}", l)),
                )
                .collect::<Vec<Captures>>()
                .iter()
                .for_each(|cap: &Captures| {
                    let parsed = cap.get(0).expect("Empty capture group").as_str();

                    match l % 7 {
                        0 => {
                            monkey.name = parsed.parse::<u64>().expect("Couldn't parse monkey name")
                        }
                        1 => {
                            monkey
                                .worry_levels
                                .push(parsed.parse::<u64>().expect("Couldn't parse worry level"));
                            monkey.inspected_count += 1;
                        }
                        2 => {
                            if monkey.op.0 == String::new() {
                                monkey.op.0 = parsed.to_owned();
                            } else {
                                monkey.op.1 = parsed.to_owned();
                            }
                        }
                        3 => {
                            monkey.test_against =
                                parsed.parse::<u64>().expect("Couldn't parse divisor")
                        }
                        4 => {
                            monkey.throw_to.0 = parsed
                                .parse::<u64>()
                                .expect("Couldn't parse monkey for true case")
                        }
                        5 => {
                            monkey.throw_to.1 = parsed
                                .parse::<u64>()
                                .expect("Couldn't parse monkey for false case")
                        }
                        _ => panic!(),
                    };
                });
        }
    });

    Ok(monkeys)
}

fn largest_two(values: Vec<u64>) -> (u64, u64) {
    let (mut largest, mut second_largest) = (0, 0);

    for value in values {
        if value > largest {
            second_largest = largest;
            largest = value;
        } else if value > second_largest {
            second_largest = value;
        }
    }

    (largest, second_largest)
}
