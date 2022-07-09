use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut numbers: Vec<u8> = (1..60).collect();
    let mut bonus: Vec<u8> = (1..13).collect();
    numbers.shuffle(&mut thread_rng());
    bonus.shuffle(&mut thread_rng());
    println!("Numbers: {:?}", numbers.iter().take(5).collect::<Vec<_>>());
    println!("Bonus: {:?}", bonus.iter().take(2).collect::<Vec<_>>());
}
