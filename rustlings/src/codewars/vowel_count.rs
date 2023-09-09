pub fn func(string: &str) -> usize {
    string.matches(&['a', 'e', 'i', 'o', 'u']).count()
}

#[test]
fn func_test() {
    assert_eq!(func("abracadabra"), 5);
}
