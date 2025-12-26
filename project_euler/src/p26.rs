pub fn reciprocal_cycles() {
    for d in 2..1000 {
        println!("{:.32}", (1_f64 / d as f64));
        if d == 10 {
            break;
        }
    }
}
