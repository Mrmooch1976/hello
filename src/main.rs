use rand::RngExt;


fn main() {

    let random_number = rand::rng().random_range(1..=10);
    println!("The random number is: {}", random_number);
}
