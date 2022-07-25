use std::collections::HashMap;

fn main() {
    let mut cache = Vec::new();
    let mut result;
    for n in 1..4000 {
        result = memoized_fib(n);
        cache.push(result);
        println!("result is {:?}", result);
    }
    println!("cache is {:?}", cache);
}

pub fn memoized_fib(num: usize) -> usize {
    struct Fibo {
        memo: HashMap<usize, usize>,
    }

    impl Fibo {
        fn new(num: usize) -> Fibo {
            return Fibo {
                memo: HashMap::with_capacity(num),
            };
        }

        fn get_fibo(&mut self, num: usize) -> usize {
            if num <= 2 {
                return 1;
            }

            if !self.memo.contains_key(&num) {
                let fibo_one = self.get_fibo(num - 1);
                let fibo_two = self.get_fibo(num - 2);

                self.memo.entry(num).or_insert(fibo_one + fibo_two);
            }
            return *self.memo.get(&num).unwrap();
        }
    }

    let mut result = Fibo::new(num);
    return result.get_fibo(num);
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5ed66133d8c04c73d382d73a39ee177a
pub fn main_fibo() {
    let seed = (0u128, 1u128);
    let iter = std::iter::successors(Some(seed), |(x, y)| x.checked_add(*y).map(|xy| (*y, xy)));

    for fib in iter.map(|(_, i)| i) {
        println!("{}", fib);
    }
}
/*

https://codereview.stackexchange.com/a/253969

   |
help: you can convert an `i64` to a `usize` and panic if the converted value doesn't fit
   |
15 |     let u: usize = n.try_into().unwrap();

*/

/*
https://codereview.stackexchange.com/questions/204555/recursive-fibonacci-in-rust-with-memoization#:~:text=Implement%20a%20generic,with%20a%20Vec%3F

Implement a generic Fibonacci sequence in Rust without using Copy trait => https://codereview.stackexchange.com/q/130042/32521
How to swap two variables? => https://stackoverflow.com/q/31798737/155423
How to avoid excessive cloning in Rust? => https://stackoverflow.com/q/40965230/155423
Is it possible to use a fold with a Vec? => https://stackoverflow.com/q/27760022/155423

*/
