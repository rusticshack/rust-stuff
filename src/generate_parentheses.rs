pub(crate) fn generate_parentheses(n: i32) -> Vec<String> {
    let mut parens: Vec<String> = Vec::new();
    let s: String = String::from("");
    helper(n, 0, 0, s, &mut parens);
    return parens;
}


/// Recursive backtracking helper to fill the vector with valid parens
///
/// Each recursive call builds up the supplied string further by allocating
/// a new one with an additional paren added.
fn helper(n: i32, l: i32, r: i32, s: String, parens: &mut Vec<String>) {
    if l + r == 2 * n {
        parens.push(s);
    } else if l == r {
        helper(n, l + 1, r, format!("{}{}", s, "("), parens);
    } else {
        if l < n {
            helper(n, l + 1, r, format!("{}{}", s, "("), parens);
        }
        helper(n, l, r + 1, format!("{}{}", s, ")"), parens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_generates_parentheses() {
        assert_eq!(generate_parentheses(1), vec!["()"]);
        assert_eq!(generate_parentheses(2), vec!["(())", "()()"]);
        assert_eq!(generate_parentheses(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}