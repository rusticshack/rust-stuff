pub(crate) fn fibonnaci(n: u128) -> u128 {
    if n == 1 || n == 2 {
        return 1;
    }

    let mut last:u128 = 0;
    let mut curr:u128 = 1;
    let mut sum:u128 = 0;
    for _x in 3..=n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    return sum;
}