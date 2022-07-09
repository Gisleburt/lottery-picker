use std::num::ParseIntError;
use lottery_picker::{choose_lucky_stars, choose_numbers};
use structopt::StructOpt;

fn str_to_vec(src: &str) -> Result<Vec<u8>, ParseIntError> {
    src.split(',').map(|s| s.parse()).collect()
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    numbers: String,
    #[structopt(short, long)]
    lucky: String,
}

fn main() -> Result<(), ParseIntError> {
    let opt = Opt::from_args();
    let numbers = str_to_vec(&opt.numbers)?;
    let lucky = str_to_vec(&opt.lucky)?;
    println!("Numbers: {:?}", choose_numbers(numbers));
    println!("Bonus: {:?}", choose_lucky_stars(lucky));
    Ok(())
}
