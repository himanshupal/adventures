use std::time::Instant;

struct Series(u32, u32);

impl Iterator for Series {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += self.1;
        self.1 += 1;
        Some(self.0)
    }
}

fn factors_count(n: u32) -> u32 {
    let rooted = (n as f32).sqrt() as u32;
    let mut count = 1;

    for i in 1..=rooted {
        if n % i == 0 {
            count += if n / i == i { 1 } else { 2 }
        }
    }

    count
}

pub fn highly_divisible_triangular_number() {
    let mut series = Series(0, 1);
    let now = Instant::now();

    loop {
        if let Some(num) = series.next() {
            if factors_count(num) >= 500 {
                return println!("12: Found after {} seconds: {num}", now.elapsed().as_secs());
            }
        }
    }
}
