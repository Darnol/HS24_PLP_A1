use std::cmp::Ord;

fn main() {

    let mut array_to_sort: [isize; 5] = [5,4,3,2,1];

    let right = array_to_sort.len() - 1;

    quicksort(&mut array_to_sort, 0, right);

    println!("{:?}", array_to_sort);
}

/// Any type T that implements the Ord trait can be sorted using this function
/// By default, most primitive types implement the Ord trait in Rust
fn quicksort(array: &mut [isize], left: usize, right: usize) {

    println!("quicksort: Array {:?}, left {} right {}", array, left, right);

    // Base case: if the array has less than 2 elements, it is already sorted
    if left >= right {
        return;
    }
    
    // Otherwise, we partition the array and recursively sort the two halves
    let pivot_index: usize = partition(array, left, right);

    println!("quicksort: Found pivot index at {}", pivot_index);
    
    // Recursively sort the left half, but ONLY IF pivot_index is greater than 0
    if pivot_index > 0 {
        quicksort(array, left, pivot_index-1);
    }
    // partition the right half
    quicksort(array, pivot_index+1, right);
    
}

/// Partition the array into two halves such that all elements in the left half are less than the pivot
/// Pivot: I implement the "simple" pivot selection strategy that is the pivot is the rightmost element in the array
/// We use array slices which allows us to make use of the swap() method
fn partition(array: &mut [isize], left: usize, right: usize) -> usize {
    let pivot_index: usize = right;
    
    // i is the index where the pivot will end up
    let mut i: isize = left as isize - 1;

    // Loop till right-1 because we use the rightmost value as the pivot. Since Rust will not include right, this is fine
    for j in left..right {

        println!("j : {j}, array[j]: {}", array[j]);

        if array[j] <= array[pivot_index] {

            println!("Swapping {} and {}", array[i as usize], array[j]);

            i += 1;
            array.swap(i as usize, j);
        }
    }
    
    // Swap the pivot with the element at index i+1
    array.swap((i+1) as usize, pivot_index);

    println!("Array after partition: {:?}", array);

    // We want to return the index where the pivot ended up
    return (i+1) as usize;
}