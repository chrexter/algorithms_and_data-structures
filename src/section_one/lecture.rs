/**
  Write a function that calculates the sum of
  all numbers from 1 up to (and including) some
  number n.
*/

/// Big O: three simple operation regardless of the size of n.
/// Time Complexity: Constant
pub fn add_up_simplfied(number: u32) -> u32 {
    number * (number + 1) / 2
}

/// Time Complexity: Linear
pub fn add_up_to(number: i32) -> i32 {
    let mut total = 0;

    for idx in 1..=number {
        total += idx;
    }

    return total;
}

/// Time Complexity: Linear
pub fn count_up_and_down(number: i8) -> () {
    println!("Going up!");

    for idx in 0..number {
        println!("{idx}");
    }

    println!("At the top!\nGoing down...");

    for idx in (0..number).rev() {
        println!("{idx}");
    }

    println!("Back down. Bye!");
}

/// Time Complexity: Linear
pub fn while_count_up_and_down(number: i8) -> () {
    let mut i = 0;

    while i < number {
        println!("{i}");
        i += 1;
    }

    println!("At the top!\nGoing down...");

    let mut j = number;

    while j > 0 {
        j -= 1;
        println!("{j}");
    }

    println!("Back down. Bye!");
}

pub fn print_all_pairs(number: i8) -> () {
    for idx in 0..number {
        for idxs in 0..number {
            println!("{idx}, {idxs}");
        }
    }
}
