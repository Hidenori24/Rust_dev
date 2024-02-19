pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

pub fn sub(x: i32, y: i32) -> i32 {
    return x - y;
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(0, 2));
    assert_eq!(3, add(2, 1));
    assert_eq!(2, add(4, -2));
}

#[test]
fn test_sub() {
    assert_eq!(0, sub(1, 1));
    assert_eq!(3, sub(5, 2));
    assert_eq!(-2, sub(2, 4));
}

fn main() {
    let added = add(1, 3);

    println!("add is {}.", added);
}
