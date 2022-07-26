/// sorting algorithm       time:   [4.0485 ns 4.0789 ns 4.1194 ns] change: [-0.6276% +0.4626% +1.7415%] (p = 0.47 > 0.05)
/// /// No change in performance detected.  /// Found 11 outliers among 100 measurements (11.00%) ///   3 (3.00%) high mild ///   8 (8.00%) high severe
pub(crate) fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::bubble_sort::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [6, 2, 4, 1, 9, -2, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
    }
}
