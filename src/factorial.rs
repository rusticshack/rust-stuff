pub(crate) fn factorial(x: i32) -> i32 {
    return if x == 1 {
        1
    } else {
        x * factorial(x - 1)
    }
}
