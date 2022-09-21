fn main() {
    println!("Hello, world!");
}

#[test]
fn test_plus() {
    assert_eq!(1 + 2, 3);
    assert_ne!(1 + 1, 3);
}

#[test]
fn test_eq() {
    assert!(1 == 1);
}
