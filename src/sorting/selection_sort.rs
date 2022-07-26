/// sorting algorithm       time:   [7.8252 ns 7.8345 ns 7.8431 ns]
///                        change: [+89.837% +91.856% +93.251%] /// (p = 0.00 < 0.05) ///                        Performance has regressed.  /// Found 3 outliers among 100 measurements (3.00%)
///  3 (3.00%) high mild
pub(crate) fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::selection_sort::selection_sort;

    #[test]
    fn test_selection_sort() {
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
}
