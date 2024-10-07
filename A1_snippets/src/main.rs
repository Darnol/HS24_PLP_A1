// NOTE: PartialOrd is enough for our quicksort. Ord (total ordering) is not implemented for floating point numbers for example
use std::cmp::PartialOrd;

// Performance: I implemented a simple Lomuto quicksort algorithm. Algorithm wise there would be more efficient ways to implement quicksort.
// The lomuto algorithm has a high time complexity if the array is already sorted.
// A cleverer pivot chosing schema (for example middle of three random elements) could be implemented to improve performance
// Regarding the Rust specific implementation: I use array slices, they have the convenient swap method.
// 


fn main() {

    let mut array_to_sort_int: [isize; 5] = [5,4,3,2,1];
    println!("Sort int array {:?}", array_to_sort_int);
    let right = array_to_sort_int.len() - 1;
    quicksort(&mut array_to_sort_int, 0, right);
    println!("Sorted array int {:?}", array_to_sort_int);

    let mut array_to_sort_float: [f64; 5] = [4.2,6.77,1.34,8.94,6.34];
    println!("Sort float array {:?}", array_to_sort_float);
    let right = array_to_sort_float.len() - 1;
    quicksort(&mut array_to_sort_float, 0, right);
    println!("Sorted array float {:?}", array_to_sort_float);

    let mut array_to_sort_str: [&str; 5] = ["apple", "banana", "apostrophe", "xavier", "orange"];
    println!("Sort str array {:?}", array_to_sort_str);
    let right = array_to_sort_str.len() - 1;
    quicksort(&mut array_to_sort_str, 0, right);
    println!("Sorted array str {:?}", array_to_sort_str);

    // Interesting: Tuple in Rust implements PartialOrd if all elements in the tuple implement PartialOrd
    let mut array_to_sort_tuple: [(isize, &str); 5] = [(5, "apple"), (2, "banana"), (3, "apostrophe"), (2, "xavier"), (1, "orange")];
    println!("Sort tuple array {:?}", array_to_sort_tuple);
    let right = array_to_sort_tuple.len() - 1;
    quicksort(&mut array_to_sort_tuple, 0, right);
    println!("Sorted array tuple {:?}", array_to_sort_tuple);

    // Special cases
    let mut array_empty: [isize; 0] = [];
    println!("Sort empty array {:?}", array_empty);
    let right = array_empty.len() - 1;
    quicksort(&mut array_empty, 0, right);
    println!("Sorted empty array {:?}", array_empty);

}

/// Any type T that implements the PartialOrd trait can be sorted using this function
/// I implemented a simple Lomuto quicksort algorithm, where the pivot is the rightmost element in the array
/// NOTE: Rust requires us to specify usize for any indexing variable. This leads to some type casting problems, because most quicksort implementations use negative indices at some point
fn quicksort<T: PartialOrd>(array: &mut [T], left: usize, right: usize) {

    // println!("quicksort: Array {:?}, left {} right {}", array, left, right);

    // Base case: if the array has less than 2 elements, it is already sorted
    if left >= right {
        return;
    }
    
    // Otherwise, we partition the array and recursively sort the two halves
    let pivot_index: usize = partition(array, left, right);

    // println!("quicksort: Found pivot index at {}", pivot_index);
    
    // Recursively sort the left half, but ONLY IF pivot_index is greater than 0. Otherwise we have a isize/usize casting problem
    if pivot_index > 0 {
        quicksort(array, left, pivot_index-1);
    }
    // partition the right half
    quicksort(array, pivot_index+1, right);
    
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