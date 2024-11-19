// ---- Intro to Big O ----

// ---- Timing Our Code ----

/**
 * Write a function that calculates the sum of
 * all numbers from 1 up to (and including) some
 * number n.
*/
pub fn add_up_to(number: i32) -> i32 {
    let mut total = 0;

    for idx in 1..=number {
        total += idx;
    }

    return total;
}

pub fn add_up_simplfied(number: u32) -> u32 {
    number * (number + 1) / 2
}
