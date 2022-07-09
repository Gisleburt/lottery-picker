use rand::thread_rng;
use rand::seq::SliceRandom;

fn choose_from_pool(mut pool: Vec<u8>, count: usize) -> Vec<u8> {
    pool.shuffle(&mut thread_rng());
    let mut chosen: Vec<_> = pool.iter().take(count).map(|i| *i).collect();
    chosen.sort();
    chosen
}

pub fn choose_numbers(pool: Vec<u8>) -> Vec<u8> {
    choose_from_pool(pool, 5)
}

pub fn choose_lucky_stars(pool: Vec<u8>) -> Vec<u8> {
    choose_from_pool(pool, 2)
}
