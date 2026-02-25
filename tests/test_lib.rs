use trust::{add, divide, multiply, subtract};

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3)
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(2, 1), 1)
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(1, 2), 2)
}

#[test]
fn test_divide() {
    assert_eq!(divide(1, 2), 0)
}
