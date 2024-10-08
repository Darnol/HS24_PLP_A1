// NOTE: PartialOrd is enough for our quicksort. Ord (total ordering) is not implemented for floating point numbers for example
use std::cmp::PartialOrd;

// Performance: I implemented a simple Lomuto quicksort algorithm. Algorithm wise there would be more efficient ways to implement quicksort for sure.
// The lomuto algorithm has a high time complexity if the array is already sorted.
// A cleverer pivot chosing schema (for example middle of three random elements) could be implemented to improve performance
// Regarding the Rust specific implementation: I use array slices, they have the convenient swap method.
// By using arrays and mutable references, the whole operation should be relatively memory efficient I think, since not much data is copied in memory.

fn main() {

    let mut array_to_sort_int: [isize; 5] = [5,4,3,2,1];
    println!("Sort int array {:?}", array_to_sort_int);
    quicksort(&mut array_to_sort_int);
    println!("Sorted array int {:?}", array_to_sort_int);

    let mut array_to_sort_float: [f64; 5] = [4.2,6.77,1.34,8.94,6.34];
    println!("Sort float array {:?}", array_to_sort_float);
    quicksort(&mut array_to_sort_float);
    println!("Sorted array float {:?}", array_to_sort_float);

    let mut array_to_sort_str: [&str; 5] = ["apple", "banana", "apostrophe", "xavier", "orange"];
    println!("Sort str array {:?}", array_to_sort_str);
    quicksort(&mut array_to_sort_str);
    println!("Sorted array str {:?}", array_to_sort_str);

    // Interesting: Tuple in Rust implements PartialOrd if all elements in the tuple implement PartialOrd
    let mut array_to_sort_tuple: [(isize, &str); 5] = [(5, "apple"), (2, "banana"), (3, "apostrophe"), (2, "xavier"), (1, "orange")];
    println!("Sort tuple array {:?}", array_to_sort_tuple);
    quicksort(&mut array_to_sort_tuple);
    println!("Sorted array tuple {:?}", array_to_sort_tuple);

    // Special cases
    let mut array_empty: [isize; 0] = [];
    println!("Sort empty array {:?}", array_empty);
    quicksort(&mut array_empty);
    println!("Sorted empty array {:?}", array_empty);

    let mut array_len_1: [isize; 1] = [100];
    println!("Sort len 1 array {:?}", array_len_1);
    quicksort(&mut array_len_1);
    println!("Sorted len 1 array {:?}", array_len_1);

    let mut array_len_2_identical: [isize; 2] = [200, 200];
    println!("Sort len 2 identical array {:?}", array_len_2_identical);
    quicksort(&mut array_len_2_identical);
    println!("Sorted len 2 identical array {:?}", array_len_2_identical);

}

/// This is the "wrapper" entry point for any quicksort application. Does not require to specify left and right. Otherwise calling the function in main is a bit cumbersome
/// I got this idea from here: https://www.alxolr.com/articles/quicksort-algorithm-implemented-in-rust-language-with-an-easy-to-remember-partitioning-function
fn quicksort<T: PartialOrd>(array: &mut [T]) {
    _quicksort(array, 0, (array.len() as isize) - 1);
}

/// Any type T that implements the PartialOrd trait can be sorted using this function
/// I implemented a simple Lomuto quicksort algorithm, where the pivot is the rightmost element in the array
/// NOTE: Rust requires us to specify usize for any indexing variable. This leads to some type casting problems, because most quicksort implementations use negative indices at some point.
fn _quicksort<T: PartialOrd>(array: &mut [T], left: isize, right: isize) {

    // println!("quicksort: Array {:?}, left {} right {}", array, left, right);

    // Base case: if the array has less than 2 elements, it is already sorted
    if left >= right {
        return;
    }

    let left = left as usize;
    let right = right as usize;
    
    // Otherwise, we partition the array and recursively sort the two halves
    let pivot_index: usize = partition(array, left, right);

    // println!("quicksort: Found pivot index at {}", pivot_index);
    
    // Recursively sort the left half, but ONLY IF pivot_index is greater than 0. Otherwise we have a isize/usize casting problem
    if pivot_index > 0 {
        _quicksort(array, left as isize, (pivot_index-1) as isize);
    }
    // partition the right half
    _quicksort(array, (pivot_index+1) as isize, right as isize);
    
}

/// Partition the array into two halves such that all elements in the left half are less than the pivot
/// We use array slices which allows us to make use of the swap() method
fn partition<T: PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    let pivot_index: usize = right;
    
    // i is the index of the last element that is smaller than the current pivot after the pass through the array
    let mut i: isize = left as isize - 1;

    // Loop till the second last element. Since rust does not include the right bound in the slice, we dont subtract 1 from right
    for j in left..right {

        // println!("j : {j}, array[j]: {}", array[j]);

        if array[j] <= array[pivot_index] {

            // println!("Swapping {} and {}", array[i as usize], array[j]);

            i += 1;
            array.swap(i as usize, j); // At this point, we can be sure that i >= 0, so we can safely cast it to usize
        }
    }
    
    // Swap the pivot with the element at index i+1
    array.swap((i+1) as usize, pivot_index);

    // println!("Array after partition: {:?}", array);

    // We want to return the index where the pivot ended up
    return (i+1) as usize; // Even if we executed the inner loop 0 times, i will at least be 0, so the cast should be safe
}