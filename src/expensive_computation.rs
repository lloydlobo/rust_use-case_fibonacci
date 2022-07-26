use super::write_output_bytes;
use crate::write_output_txt;

pub(crate) fn expensive_computation() -> usize {
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
