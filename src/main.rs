use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=10);
    println!("The random number is: {}", random_number);
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
    }
}
