use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let random_number: i32 = rng.random_range(1..=100);
    println!("Random number: {}", random_number);
}