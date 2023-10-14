use std::time::Instant;

fn move_ahead(rows: u64, columns: u64) -> u64 {
    match (rows, columns) {
        x if x.0 > 0 && x.1 > 0 => move_ahead(rows - 1, columns) + move_ahead(rows, columns - 1),
        x if x.0 > 0 => move_ahead(rows - 1, columns),
        x if x.1 > 0 => move_ahead(rows, columns - 1),
        _ => 1,
    }
}

pub fn lattice_paths() {
    let now = Instant::now();
    let paths = move_ahead(20, 20);

    println!(
        "15: Found after {} seconds: {}",
        now.elapsed().as_secs(),
        paths
    );
}
