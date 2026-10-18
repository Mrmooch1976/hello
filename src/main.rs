use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=10);
    println!("The random number is: {}", random_number);
    println!("Is even? {}", is_even(random_number));
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::is_even;

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
    }
}
