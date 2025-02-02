pub(crate) fn factorial(x: i32) -> i32 {
    return if x == 1 {
        1
    } else {
        x * factorial(x - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(10), 3628800);
    }
}
