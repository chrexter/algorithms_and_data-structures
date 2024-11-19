mod section_one;

use section_one::{
    add_up_simplfied, add_up_to, count_up_and_down, print_all_pairs, while_count_up_and_down,
};

fn main() {
    let sum_up = add_up_to(100);
    println!("Sum up value: {}", sum_up);

    let simplified = add_up_simplfied(6);
    println!("Add up simplified: {}", simplified);

    // count_up_and_down(10);
    // while_count_up_and_down(10);
    print_all_pairs(10);
}
