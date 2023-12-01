use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("src/input")?;
    let buf_reader = BufReader::new(file);
    let mut counter = 0;
    buf_reader.lines().for_each(|line| {
        let line = line.unwrap();
        let mut string = String::new();
        for c in line.clone().into_bytes().iter() {
            if let b'0'..=b'9' = c {
                string.push(*c as char);
                break;
            }
        }

        for c in line.into_bytes().iter().rev() {
            if let b'0'..=b'9' = c {
                string.push(*c as char);
                break;
            }
        }
        println!("{}", string);
        counter += string.parse::<i32>().unwrap();
    });

    println!("{}", counter);

    Ok(())
}
