#[no_mangle]
pub extern fn fib(x: i32) -> i32 {
    if x < 1 {
        return 1;
    } else {
        return fib(x - 1) + fib(x - 2);
    }
}


#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
