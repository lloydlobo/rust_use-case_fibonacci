use std::fs::File;
use std::io::prelude::*;

pub(crate) fn write_output() -> std::io::Result<()> {
    let data = b"some bytes";

    let mut position = 0;
    let mut buffer = File::create("output.txt")?;

    while position < data.len() {
        let bytes_written = std::io::Write::write(&mut buffer, &data[position..])?;
        position += bytes_written;
    }
    Ok(())
}
