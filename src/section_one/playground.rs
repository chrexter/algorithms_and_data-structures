/**
  Write a function that calculates the sum of
  all numbers from 1 up to (and including) some
  number n.
*/

/// Big O: three simple operation regardless of the size of n.
/// Time Complexity: Constant - O(1)
pub fn add_up_simplfied(number: u32) -> u32 {
    number * (number + 1) / 2
}

/// Time Complexity: Linear  - O(n)
pub fn add_up_to(number: i32) -> i32 {
    let mut total = 0;

    for idx in 1..=number {
        total += idx;
    }

    return total;
}

/// Time Complexity: Linear  - O(n)
pub fn _count_up_and_down(number: i8) -> () {
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

/// Time Complexity: Linear - O(n)
pub fn _while_count_up_and_down(number: i8) -> () {
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

// Time Complexity: Quadratic - O(n²)
pub fn _print_all_pairs(number: i8) -> () {
    for idx in 0..number {
        for idxs in 0..number {
            println!("{idx}, {idxs}");
        }
    }
}

/**
 * Time Complexity: The time complexity of this function is O(n),
 * where n is the maximum of 5 and the input value. It's linear because
 * the number of iterations directly depends on the larger of the two
 * values. While the function looks like it may have a constant time
 * complexity due to the 5 being ever-present, the input value can be
 * an arbitrary value, so its runtime does scale with the input value.
*/
pub fn _log_at_least_5(value: u8) -> () {
    for i in 1..=u8::max(5, value) {
        println!("{i}");
    }
}

/// Time Complexity: The time complexity of this function is O(1),
/// where 5 is the maximum of n and the input value. It's constant because
/// the number of iterations directly depends on 5.
pub fn log_at_most_5(value: u8) -> () {
    for idx in 1..=u8::min(5, value) {
        println!("{idx}");
    }
}

// Space Complexity: O(1)
pub fn sum_ups(payload: [u8; 5]) -> () {
    let mut value: u8 = 0; // - constant space complexity

    for item in payload {
        value += item;
    }

    println!("{value}");
}

// Space Complexity: O(n)
pub fn double_ups(payload: Vec<u8>) -> () {
    let mut new_array: Vec<u8> = Vec::new();

    for item in payload {
        new_array.push(2 * item);
    }

    println!("{new_array:?}")
}

// Convert temperatures between Fahrenheit and Celsius.
/**
* Business Logic:
*  Temperature conversions use the following formulas:
   - Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
   - Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9

   Only digits 0 to 9 and a single decimal point (".") are acceptable as numeric inputs.
   Attempted input of other characters into a numeric field may lead to an incorrect result.
* */

pub fn temperature_converter(temp: u32) -> () {}
