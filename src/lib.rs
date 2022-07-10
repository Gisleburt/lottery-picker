pub mod online;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn choose_from_pool(mut pool: Vec<u8>, count: usize) -> Vec<u8> {
    pool.shuffle(&mut thread_rng());
    let mut chosen: Vec<_> = pool.iter().take(count).map(|i| *i).collect();
    chosen.sort();
    chosen
}

/// Choose 5 numbers from a pool of numbers
/// ```rust
/// use lottery_picker::choose_numbers;
///
/// let pool: Vec<_> = (1..100).collect();
/// assert_eq!(choose_numbers(pool).len(), 5);
/// ```
pub fn choose_numbers(pool: Vec<u8>) -> Vec<u8> {
    choose_from_pool(pool, 5)
}

/// Choose 2 numbers from a pool of numbers
/// ```rust
/// use lottery_picker::choose_lucky_stars;
///
/// let pool: Vec<_> = (1..100).collect();
/// assert_eq!(choose_lucky_stars(pool).len(), 2);
/// ```
pub fn choose_lucky_stars(pool: Vec<u8>) -> Vec<u8> {
    choose_from_pool(pool, 2)
}
