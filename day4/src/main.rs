use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("src/input").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut count = 0;

    buf_reader.lines().for_each(|line| {
        let line = line.unwrap();
        let nums = line.split(':').collect::<Vec<&str>>()[1]
            .split('|')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let (winning_numbers, numbers_have) = (
            nums[0]
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.trim().parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>(),
            nums[1]
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.trim().parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>(),
        );

        let mut inner_count = 0;
        winning_numbers.into_iter().for_each(|x| {
            if numbers_have.contains(&x) && x != 0 {
                inner_count += 1;
            }
        });
        
        if inner_count > 0 {
            count += 2_u32.pow(inner_count - 1);
        }
    });

    println!("Part 1: {}", count);
}

fn part2() {}
