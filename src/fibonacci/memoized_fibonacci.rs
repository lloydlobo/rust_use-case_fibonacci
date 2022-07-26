use std::collections::HashMap;

pub(crate) fn memoized_fibonacci(num: usize) -> usize {
    struct Fibo {
        memo: HashMap<usize, usize>,
    }

    impl Fibo {
        pub fn new(num: usize) -> Fibo {
            return Fibo {
                memo: HashMap::with_capacity(num),
            };
        }
        pub fn get_fibo(&mut self, num: usize) -> usize {
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
