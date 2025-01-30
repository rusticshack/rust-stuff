mod factorial;
mod fibonnaci;
mod generate_parentheses;

fn main() {
    println!("{}", factorial::factorial(10));
    println!("{}", fibonnaci::fibonnaci(12));
    println!("{:?}", generate_parentheses::generate_parentheses(5));
}