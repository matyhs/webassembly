#[no_mangle]
extern fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
extern fn sub(x: i32, y: i32) -> i32 {
    x - y
}

#[no_mangle]
extern fn mul(x: i32, y: i32) -> i32 {
    x * y
}

#[no_mangle]
extern fn div(x: i32, y: i32) -> i32 {
    match (x).checked_div(y) {
        Some(num) => num,
        None => 0
    }
}