pub fn climb_stairs(n: i32) -> i32 {
    if n > 2 {
        let (mut current, mut next) = (1, 2);

        for i in 2..n {
            let t = current + next;
            current = next;
            next = t;
        }

        next
    } else {
        n
    }
}

#[test]
fn climb_stairs_test() {
    assert_eq!(climb_stairs(0), 0);
    assert_eq!(climb_stairs(1), 1);
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(5), 8);
    assert_eq!(climb_stairs(6), 13);
}
