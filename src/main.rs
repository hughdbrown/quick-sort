mod prng;
mod quicksort;
mod util;

use prng::{
    make_random_vec,
};
use util::{
    get_i32,
    check_sorted,
    print_vec,
};
use quicksort::{
    quick_sort,
};


fn main() {
    let max = get_i32("Maximum value for array: ");
    let count = get_i32("Number of random values to sort: ");
    let mut values: Vec<i32> = make_random_vec(count, 0, max);
    print_vec(&values, 20);
    quick_sort(&mut values);
    print_vec(&values, 20);
    println!("Sorted: {}", check_sorted(&values));
}
