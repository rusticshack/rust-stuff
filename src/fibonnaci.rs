pub(crate) fn fibonnaci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonnaci_helper(n)
    }
}

fn fibonnaci_helper(n: u128) -> u128 {
    let mut last:u128 = 1;
    let mut curr:u128 = 1;
    let mut sum:u128 = 0;
    for _x in 3..=n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fibonnaci() {
        assert_eq!(fibonnaci(0), 0);
        assert_eq!(fibonnaci(1), 1);
        assert_eq!(fibonnaci(2), 1);
        assert_eq!(fibonnaci(12), 144);
    }
}