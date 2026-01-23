use std::{
    env::args,
    fs::{File, read_to_string},
    io::prelude::*,
};

fn main() -> Result<(), std::io::Error> {
    let arguments = args().collect::<Vec<String>>();

    if arguments.len() >= 3 {
        let mut target = File::create(&arguments.last().unwrap())?;

        for input_path in arguments[1..arguments.len() - 1].iter() {
            let source = read_to_string(&input_path)?;
            let mut block_length: usize = 0;

            for line in source.lines() {
                let trimmed = line.trim();

                if trimmed.starts_with("///") {
                    target.write_fmt(format_args!("{}\n", &trimmed[3..].trim_start()))?;
                    block_length += 1;
                } else if block_length != 0 {
                    target.write_all("\n".as_bytes())?;
                    block_length = 0;
                }
            }
        }

        Ok(())
    } else {
        Err(std::io::ErrorKind::InvalidInput.into())
    }
}
