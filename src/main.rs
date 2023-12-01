use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("src/input")?;
    let buf_reader = BufReader::new(file);
    let mut counter = 0;

    buf_reader.lines().for_each(|line| {
        let line = line.unwrap();

        counter += find_number_in_string(line);
    });

    println!("{}", counter);

    Ok(())
}

fn find_number_in_string(string: String) -> i32 {
    let numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut string_aux = String::new();

    for i in 0..string.len() {
        let mut found = false;

        for num in numbers {
            let max = if i + num.0.len() > string.len() {
                string.len()
            } else {
                i + num.0.len()
            };
            let string_aux2 = &string[i..max];

            if let b'0'..=b'9' = string.chars().nth(i).unwrap() as u8 {
                string_aux.push(string.chars().nth(i).unwrap());
                found = true;
                break;
            } else if string_aux2.contains(num.0) {
                string_aux.push(num.1);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    for i in (0..string.len()).rev() {
        let mut found = false;

        for num in numbers {
            let min = if num.0.len() > i { 0 } else { i - num.0.len() };
            let string_aux2 = &string[min..=i];

            if let b'0'..=b'9' = string.chars().nth(i).unwrap() as u8 {
                string_aux.push(string.chars().nth(i).unwrap());
                found = true;
                break;
            } else if string_aux2.contains(num.0) {
                string_aux.push(num.1);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    string_aux.parse::<i32>().unwrap_or(0)
}
