pub use std::iter::{successors, Successors};

/// `checked_add_map` takes two `u128`s, adds them, and returns the result as a tuple of the two inputs
/// and the result
///
/// Arguments:
///
/// * `x`: &u128 - the first number to add
/// * `y`: The value to add to x.
///
/// Returns:
///
/// A tuple of the form (y, xy)
pub(crate) fn checked_add_map(x: &u128, y: &u128) -> Option<(u128, u128)> {
    x.checked_add(*y).map(|xy| (*y, xy))
}

/// `print_iterate_map` takes an iterator of tuples of `u128`s, and prints the second element of each
/// tuple
///
/// Arguments:
///
/// * `iterate`: impl Iterator<Item = (u128, u128)>
pub(crate) fn print_iterate_map(iterate: impl Iterator<Item = (u128, u128)>) -> () {
    let map: _ = Iterator::map(iterate, |(_, i): (u128, u128)| i);
    for fibonacci_number in map {
        println!("{}", fibonacci_number);
    }
}

/// `first` returns a tuple of two `u128`s, the first of which is `0` and the second of which is `1`
///
/// Returns:
///
/// A tuple of two u128s.
pub(crate) fn first() -> Option<(u128, u128)> {
    let seed: (u128, u128) = (0u128, 1u128);
    let first: Option<(u128, u128)> = Some(seed);
    first
}

/////////////////////////////////////////////////////////////////////////
// Iterate fibonacci
/////////////////////////////////////////////////////////////////////////

/// `iter_fibonacci` takes an optional tuple of two `u128`s, and returns an iterator of `u128`s
/// Prints out Fibonacci numbers where each successive item is computed based on the preceding one.
///
/// # Arguments:
/// * `first`: Option<(u128, u128)>
///
/// # Examples
///
/// Basic usage:
/// ## Successors
/// ```
/// use std::iter::successors;
/// let powers_of_10 = successors(Some(1_u16), |n| n.checked_mul(10));
/// assert_eq!(powers_of_10.collect::<Vec<_>>(), &[1, 10, 100, 1_000, 10_000]);
/// ```
pub(crate) fn iter_fibonacci(first: Option<(u128, u128)>) -> () {
    let iterate: _ = successors(first, |(x, y)| checked_add_map(x, y));
    print_iterate_map(iterate);

    // iterate
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5ed66133d8c04c73d382d73a39ee177a

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_u128_some_unwrap() {
        // let first_arg_0_1 = first();
        let num_1: u128 = 0 as u128;
        let num_2: u128 = 1 as u128;
        let mock_1 = Some(num_1);
        let mock_2 = Some(num_2);
        assert_eq!(Some(num_1), mock_1);
        assert_eq!(Some(num_2), mock_2);
        assert_eq!(num_1, mock_1.unwrap());
        assert_eq!(num_2, mock_2.unwrap());
    }

    #[test]
    fn test_first_fn() {
        let f: Option<(u128, u128)> = first();
        assert_eq!(f, first())
    }

    #[test]
    fn test_iter_fibonacci() {
        assert_eq!(iter_fibonacci(first()), ());
    }
}
