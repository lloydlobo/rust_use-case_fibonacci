#![allow(unused_imports)]
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
    for i in 0..20 {
        let res = get_cached_val();
        println!("i: {}, res: {:?}", i, res);
        if i > 1 {
            println!(" i: {}", i);
        } else {
            println!("i: {}", i);
        }
    }
    loop_fibo_memoize();
}

/////////////////////////////////////////////////////////////////////////
// HELPER FUNCTION
/////////////////////////////////////////////////////////////////////////

pub(crate) fn write_output_bytes() -> std::io::Result<()> {
    let data = b"some output bytes";

    let mut position = 0;
    let mut buffer = File::create("output_bytes.txt")?;

    while position < data.len() {
        let bytes_written = std::io::Write::write(&mut buffer, &data[position..])?;
        position += bytes_written;
    }
    Ok(())
}

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
    println!("expensive_computation #\\{}, res: {:?}", counter, res); // $ res: Ok(())
    let res_out = write_output_bytes();
    counter += 1;
    println!("expensive_computation #\\{}, res: {:?}", counter, res_out);

    usize_res
}

// unsafe fn unsafe_fn() {} // extern "C" { //     fn unsafe_extern_fn(); //     static BAR: *mut u32; // } // trait SafeTraitWithUnsafeMethod { //     unsafe fn unsafe_method(&self); // } // struct S; // impl S { //     unsafe fn unsafe_method_on_struct() {} // }

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
