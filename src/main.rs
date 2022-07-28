#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::{fs::File, io::prelude::*, sync::Once};

/* The I/O Prelude.
   The purpose of this module is to alleviate imports of many common I/O traits by adding a glob import to the top of I/O heavy modules:
*/

/////////////////////////////////////////////////////////////////////////
//  MODULES
/////////////////////////////////////////////////////////////////////////

pub(crate) mod fibonacci;

mod write_output_txt;

/////////////////////////////////////////////////////////////////////////
//  GLOBAL VARIABLES
/////////////////////////////////////////////////////////////////////////

static mut VAL: usize = 0;
static INIT: Once = Once::new();

/////////////////////////////////////////////////////////////////////////
// MAIN FUNCTION
/////////////////////////////////////////////////////////////////////////

fn main() {
    let mut array_memo_fibo = Vec::new();
    for i in 0..9 {
        // array_memo_fibo.push(memoized_fibonacci(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
        array_memo_fibo.push(memoize_fibo(i));
    }
    println!("{:?}", array_memo_fibo);
    // let res = get_cached_val(); // println!("hello world"); // let res = write_output_bytes(40); // println!("res: {:?}", res);
}

pub fn memoize_fibo(num: u128) -> u128 {
    #[derive()]
    struct Fibo {
        memoize: HashMap<u128, usize>,
    }

    impl Fibo {
        pub fn new_hashmap_with_capacity(num: u128) -> Fibo {
            let num_usize = num.try_into().unwrap();

            return Fibo {
                memoize: HashMap::with_capacity(num_usize),
            };
        }

        pub fn fibo_prev_cur(&mut self, num: u128) -> (u128, u128) {
            let fibo_previous: u128 = self.get_fibo_for(num - 2);
            let fibo_current: u128 = self.get_fibo_for(num - 1);

            (fibo_previous, fibo_current)
        }

        pub fn get_fibo_for(&mut self, num: u128) -> u128 {
            if num <= 2 {
                return 1;
            }

            if !self.memoize.contains_key(&num) {
                let (prev, cur): (u128, u128) = self.fibo_prev_cur(num);
                let sum: usize = (prev + cur).try_into().unwrap_or_default();
                // Find the next fib or Inserts the sum if empty
                self.memoize.entry(num).or_insert(sum);
            }
            let result: usize = *self.memoize.get(&num).unwrap();

            result.try_into().unwrap_or_default()
        }
    }

    Fibo::new_hashmap_with_capacity(num).get_fibo_for(num)
}

// return (num as u128) + (num as u128);
/////////////////////////////////////////////////////////////////////////
// CURRENT FUNCTIONS
/////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////
use std::convert::{TryFrom, TryInto};

enum MyType {
    F32(f32),
    Str(String),
    USize(usize),
}

#[derive()]
enum TypeCheck {
    Add,
    Subtract,
    USize,
}

impl TypeCheck {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::USize => x - y,
        }
    }
}
// E0599 // This error occurs when a method is used on a type which does not implement it: // In this case, you need to implement the chocolate method to fix the error:
struct Mouth;

impl Mouth {
    fn chocolate(&self) {
        // We implement the `chocolate` method here.
        println!("Hmmm! I love chocolate!");
    }
}

// let x = Mouth;
// x.chocolate(); // ok!
// https://doc.rust-lang.org/std/convert/trait.From.html
// Vec<u8> -> String    String::from_utf8(v)
// https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8

fn convert_vec_to_str() {
    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());
    for _ in 0..5 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }
}
pub(crate) fn write_output_bytes(num: usize) -> std::io::Result<()> {
    let num_is_usize = num.clone();
    // usize .. //
    let usize_fibo: usize = fibonacci::memoized_fibonacci(num_is_usize);
    // u128 .. //

    let str_fibo: String = usize_fibo.to_string();
    let bytes: &[u8] = str_fibo.as_bytes();
    let data: &[u8] = bytes.as_ref();

    println!("data: {:?}, usize_fibo: {:?}", data, usize_fibo);
    let mut position: usize = 0;
    let mut buffer: File = File::create("output_bytes.txt")?;

    while position < data.len() {
        let bytes_written: usize = std::io::Write::write(&mut buffer, &data[position..])?;
        position += bytes_written;
    }
    Ok(())
}
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision

/* Accessing a `static mut` is unsafe much of the time, but if we do so
in a synchronized fashion (e.g., write once or read all) then we're
good to go!
This function will only call `expensive_computation` once, and will
otherwise always return the value returned from the first invocation.
https://doc.rust-lang.org/std/sync/struct.Once.html
*/
fn get_cached_val() -> usize {
    unsafe {
        INIT.call_once(|| {
            VAL = expensive_computation();
        });
        VAL
    }
}
fn expensive_computation() -> usize {
    let mut counter = 0;
    let usize_res: usize = 0;
    let res = write_output_txt::write_output();

    counter += 1;
    println!("computation: {}, res: {:?}", counter, res); // $ res: Ok(())
    let res_out = write_output_bytes(40);
    counter += 1;
    println!("computation: {}, res: {:?}", counter, res_out);

    usize_res
}

pub(crate) fn loop_fibo_memoize() {
    let mut cache = Vec::new();
    let mut result;
    for n in 1..100 {
        result = fibonacci::memoized_fibonacci(n);
        cache.push(result);
        // println!("result is {:?}", result);
    }
    println!("cache is {:?}", cache);
}

// fn usize_to_borrowed_u8_bytes(num: usize) -> &[u8] {
//     // let data: &[u8; 9] = b"fibonacci";
//     let num_copy: usize = num.clone();
//     let usize_fibo: usize = fibonacci::memoized_fibonacci(num_copy);
//     let str_fibo: String = usize_fibo.to_string();
//     let bytes: &[u8] = str_fibo.as_bytes();
//     let copy_bytes: &[u8] = bytes.as_ref();
//     copy_bytes // Lifetime Elision
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str, // } // impl<'a> ImportantExcerpt<'a> { //     fn level(&self) -> i32 { //         3 //     } // } // impl<'a> ImportantExcerpt<'a> { //     fn announce_and_return_part(&self, announcement: &str) -> &str { //         println!("Attention please: {}", announcement); //         self.part //     } // } // fn imp_main() { //     let novel = String::from("Call me Ishmael. Some years ago..."); //     let first_sentence = novel.split('.').next().expect("Could not find a '.'"); //     let i = ImportantExcerpt { //         part: first_sentence, //     }; // }

/* Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes.
In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
*/

// const WORDS: &'static usize = &"hello rust!".len();
// let u8_fibo: u8 = usize_fibo.try_into().unwrap(); // let count = WORDS.clone(); // const COUNT: usize = *WORDS; // const WORDS: &str = "hello convenience!"; // let bit_u8_fibo = usize_fibo.to_be_bytes();

// unsafe fn unsafe_fn() {} // extern "C" { //     fn unsafe_extern_fn(); //     static BAR: *mut u32; // } // trait SafeTraitWithUnsafeMethod { //     unsafe fn unsafe_method(&self); // } // struct S; // impl S { //     unsafe fn unsafe_method_on_struct() {} // }
fn usize_u128() {
    let size: usize = 42;
    let good: u128 = u128::try_from(size).unwrap();
    let double_good: usize = good.try_into().unwrap();
}
