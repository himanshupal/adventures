pub fn self_powers() {
    let sum = (1..=100u32).map(|n| (n as u128).pow(n)).sum::<u128>();
    println!("Sum: {sum}");
}
