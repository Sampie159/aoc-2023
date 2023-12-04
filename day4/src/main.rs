use std::collections::HashMap;
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

fn part2() {
    let file = File::open("src/input").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut cards_map: HashMap<u32, u32> = HashMap::new();
    let mut card_line = 1;

    buf_reader.lines().for_each(|line| {
        cards_map
            .entry(card_line)
            .and_modify(|count| *count += 1)
            .or_insert(1);

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

        let mut count = 0;

        winning_numbers.into_iter().for_each(|x| {
            if numbers_have.contains(&x) && x != 0 {
                count += 1;
            }
        });

        for i in 1..=count {
            let x = *cards_map.get(&(card_line)).unwrap_or(&0);
            cards_map
                .entry(card_line + i)
                .and_modify(move |count| *count += x)
                .or_insert(x);
        }

        card_line += 1;
    });

    let mut total_cards = 0;

    cards_map.into_iter().for_each(|(_, value)| {
        total_cards += value;
    });

    println!("Part 2: {}", total_cards);
}
