use rand::RngExt;

fn main() {
    let random_number = rand::rng().random_range(1..=10);
    println!("The random number is: {}", random_number);
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
    }
}
