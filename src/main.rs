mod section_one;

use section_one::{add_up_simplfied, add_up_to, double_ups, log_at_most_5, sum_ups};

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

    let payload: [u8; 5] = [1, 2, 3, 4, 10];
    sum_ups(payload);
    double_ups(Vec::from(payload));
}
