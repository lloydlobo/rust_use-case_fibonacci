pub mod fibonacci;
pub(crate) mod sorting;

pub(crate) use crate::fibonacci::iter_fibonacci::first;

pub fn fibonacci_sequence(num: usize) {
    fibonacci::memoized_fibonacci(num);
    fibonacci::iter_fibonacci(first());
    fibonacci::iter_fibonacci_return(first());
    fibonacci::main_return();
}

pub fn sort_arr<T: Ord>(arr: &mut [T]) {
    sorting::bubble_sort(arr); // faster than selection_sort
    sorting::selection_sort(arr);
}
