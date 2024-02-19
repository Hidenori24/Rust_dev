#[test]
#[should_panic]
fn assert_test() {
    assert!(true);

    assert!(false, "panic! value = {}", false);

    assert_eq!(true, true);
    assert_ne!(true, false);

    assert_eq!(true, false, "panic! value =({},{})", true, false);
}

#[test]
#[ignore]
fn ignored() {
    assert!(false);
}

fn main() {
    println!("Hello, world!");
}
