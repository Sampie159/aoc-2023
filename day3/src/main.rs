use std::fs::File;
use std::io::{BufRead, BufReader};

struct Number {
    value: u32,
    length: i32,
    x: i32,
    y: i32,
}

impl Number {
    fn new(value: u32, length: i32, x: i32, y: i32) -> Number {
        Number {
            value,
            length,
            x,
            y,
        }
    }

    pub fn symbol_adjacent(&self, symbols: Vec<Symbol>) -> bool {
        let mut adjacent = false;

        symbols.iter().for_each(|symbol| {
            if symbol.x >= self.x - 1
                && symbol.x <= self.x + 1
                && symbol.y >= self.y - 1
                && symbol.y <= self.y + self.length
            {
                adjacent = true;
            }
        });

        adjacent
    }
}

#[derive(Clone, Copy)]
struct Symbol {
    x: i32,
    y: i32,
}

impl Symbol {
    fn new(x: i32, y: i32) -> Symbol {
        Symbol { x, y }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("src/input").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut line_x = 0;

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    buf_reader.lines().for_each(|line| {
        parse_string(line.unwrap(), &mut numbers, &mut symbols, line_x);
        line_x += 1;
    });

    let sum = numbers.iter().fold(0, {
        let symbols = symbols;
        move |acc, number| {
            if number.symbol_adjacent(symbols.to_vec()) {
                acc + number.value
            } else {
                acc
            }
        }
    });

    println!("Part 1: {}", sum);
}

fn part2() {
    println!("Part 2");
}

fn parse_string(input: String, numbers: &mut Vec<Number>, symbols: &mut Vec<Symbol>, line_x: i32) {
    let mut line_y = 0;
    let input = input.into_bytes();

    while line_y < input.len() as i32 {
        if input[line_y as usize].is_ascii_digit() {
            let mut value: u32 = (input[line_y as usize] - 48).into();
            let mut length = 1;
            let y = line_y;

            while (line_y + 1) < input.len() as i32 && input[(line_y + 1) as usize].is_ascii_digit()
            {
                line_y += 1;
                value = value * 10 + ((input[line_y as usize] - 48) as u32);
                length += 1;
            }

            numbers.push(Number::new(value, length, line_x, y));
            line_y += 1;
        } else if input[line_y as usize] != b'.' {
            symbols.push(Symbol::new(line_x, line_y));
            line_y += 1;
        } else {
            line_y += 1;
        }
    }
}
