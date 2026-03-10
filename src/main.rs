use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("demo.zi")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        if line.trim().is_empty() {
            println!("Empty");
            line.clear();
            continue;
        }

        println!("{}", line);
        line.clear();
    }

    Ok(())
}
