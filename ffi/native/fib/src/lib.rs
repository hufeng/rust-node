#[no_mangle]
pub extern fn fib(n: i32) -> i32 {
  match n {
    1 => 1,
    2 => 1,
    _ => fib(n - 1) + fib(n - 2)
  }
}

#[test]
fn one() {
  assert_eq!(1, fib(1))
}

#[test]
fn two() {
  assert_eq!(1, fib(2))
}

#[test]
fn three() {
  assert_eq!(2, fib(3))
}
