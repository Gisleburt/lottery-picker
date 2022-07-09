use lottery_picker::{choose_lucky_stars, choose_numbers};

fn main() {
    let numbers: Vec<u8> = (1..60).collect();
    let lucky_stars: Vec<u8> = (1..13).collect();
    println!("Numbers: {:?}", choose_numbers(numbers));
    println!("Bonus: {:?}", choose_lucky_stars(lucky_stars));
}
