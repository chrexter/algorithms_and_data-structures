mod section_one;

use section_one::{add_up_simplfied, add_up_to};

fn main() {
    let sum_up = add_up_to(100);
    println!("Sum up value: {}", sum_up);

    let simplified = add_up_simplfied(6);
    println!("Add up simplified: {}", simplified);
}
