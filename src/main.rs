mod section_one;

use section_one::{add_up_simplfied, add_up_to, log_at_least_5, log_at_most_5, print_all_pairs};

fn main() {
    let sum_up = add_up_to(100);
    println!("Sum up value: {}", sum_up);

    let simplified = add_up_simplfied(6);
    println!("Add up simplified: {}", simplified);

    // count_up_and_down(10);
    // while_count_up_and_down(10);
    // print_all_pairs(10);

    // log_at_least_5(0);

    log_at_most_5(2);
}
