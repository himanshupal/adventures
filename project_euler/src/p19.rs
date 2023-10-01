use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
enum NumberOfDays {
    TwentyEight = 28,
    TwentyNine = 29,
    Thirty = 30,
    ThirtyOne = 31,
}

fn get_rounded_day_name(day: u32) -> Day {
    match day % 7 {
        0 => Day::Sunday,
        1 => Day::Monday,
        2 => Day::Tuesday,
        3 => Day::Wednesday,
        4 => Day::Thursday,
        5 => Day::Friday,
        6 => Day::Saturday,
        _ => unreachable!(),
    }
}

fn is_leap_year(year: usize) -> bool {
    match year {
        y if y % 100 == 0 => year % 400 == 0,
        _ => year % 4 == 0,
    }
}

fn days_in_months_of_year(year: usize) -> Vec<NumberOfDays> {
    let mut days_count = vec![];

    for m in 1..=12 {
        days_count.push(match m {
            2 => match is_leap_year(year) {
                false => NumberOfDays::TwentyEight,
                true => NumberOfDays::TwentyNine,
            },
            4 | 6 | 9 | 11 => NumberOfDays::Thirty,
            _ => NumberOfDays::ThirtyOne,
        });
    }

    days_count
}

fn starting_days_for_months(year: usize, mut next_month_starts_with: Day) -> Vec<Day> {
    let mut start_days = vec![];

    days_in_months_of_year(year)
        .into_iter()
        .enumerate()
        .for_each(|(i, days)| {
            start_days.push(next_month_starts_with);
            if i < 11 {
                next_month_starts_with =
                    get_rounded_day_name(next_month_starts_with as u32 + (days as u32) - 28);
            }
        });

    start_days
}

fn starting_day_for_year(year: usize) -> Day {
    let mut next_year_starts_on = Day::Monday;

    for y in 1900..year {
        next_year_starts_on = get_rounded_day_name(
            next_year_starts_on as u32 + if is_leap_year(y) { 366 } else { 365 } % 7,
        );
    }

    next_year_starts_on
}

pub fn counting_sundays() {
    let now = Instant::now();
    let mut count = 0;

    for year in 1901..=2000 {
        starting_days_for_months(year, starting_day_for_year(year))
            .iter()
            .for_each(|starting_day_for_month| {
                if starting_day_for_month == &Day::Sunday {
                    count += 1;
                }
            });
    }

    println!(
        "19: Found after {} nanoseconds: {}",
        now.elapsed().as_nanos(),
        count
    )
}
