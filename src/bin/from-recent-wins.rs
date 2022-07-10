use std::collections::HashSet;
use std::hash::Hash;
use lottery_picker::{choose_lucky_stars, choose_numbers};
use lottery_picker::online::{get_lucky_stars_from_records, get_numbers_from_records, get_recent_winning_numbers};
use structopt::StructOpt;

fn dedupe<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    dedupe: bool,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let records = get_recent_winning_numbers()
        .await
        .expect("Could not get recent wins");
    let mut numbers = get_numbers_from_records(&records);
    let mut lucky_stars = get_lucky_stars_from_records(&records);

    if opt.dedupe {
        dedupe(&mut numbers);
        dedupe(&mut lucky_stars);
    }

    println!("Numbers: {:?}", choose_numbers(numbers));
    println!("Bonus: {:?}", choose_lucky_stars(lucky_stars));
}
