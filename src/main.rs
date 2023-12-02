use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    id: u32,
    color_sets: Vec<(Color, u32)>,
    possible: bool,
}

impl Game {
    fn new(id: u32, color_sets: Vec<(Color, u32)>) -> Game {
        Game { id, color_sets, possible: true }
    }
}

fn main() {
    let file = File::open("src/input").expect("File not found");
    let buf_reader = BufReader::new(file);
    let mut games: Vec<Game> = Vec::new();

    let mut id = 1;

    buf_reader.lines().for_each(|line| {
        let line = line.unwrap();
        let line = line.split(':').collect::<Vec<&str>>();
        let line = line[1].split(';').collect::<Vec<&str>>();

        let mut color_sets: Vec<(Color, u32)> = Vec::new();

        line.into_iter().for_each(|color_set| {
            let color_set = color_set
                .split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|line| line.trim())
                .collect::<Vec<&str>>();

            color_set.into_iter().for_each(|line| {
                let line = line.split(' ').collect::<Vec<&str>>();
                let (amount, color) = (line[0].parse::<u32>().unwrap(), line[1]);

                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => panic!("Invalid color"),
                };

                color_sets.push((color, amount));
            });
        });

        games.push(Game::new(id, color_sets));

        id += 1;
    });

    let mut possible_count = 0;

    games.into_iter().for_each(|mut game| {
        game.color_sets.into_iter().for_each(|(color, amount)| {
            match color {
                Color::Red => {
                    if amount > 12 {
                        game.possible = false;
                    }
                }
                Color::Green => {
                    if amount > 13 {
                        game.possible = false;
                    }
                }
                Color::Blue => {
                    if amount > 14 {
                        game.possible = false;
                    }
                }
            }
        });

        if game.possible {
            possible_count += game.id;
        }
    });

    println!("{}", possible_count);
}
