use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    color_sets: Vec<(Color, u32)>,
    minimums: (u32, u32, u32),
}

impl Game {
    fn new(color_sets: Vec<(Color, u32)>) -> Game {
        Game { color_sets, minimums: (0, 0, 0) }
    }
}

fn main() {
    let file = File::open("src/input").expect("File not found");
    let buf_reader = BufReader::new(file);
    let mut games: Vec<Game> = Vec::new();

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

        games.push(Game::new(color_sets));
    });

    let mut sum_power = 0;

    games.into_iter().for_each(|mut game| {
        game.color_sets.into_iter().for_each(|(color, amount)| {
            match color {
                Color::Red => game.minimums.0 = std::cmp::max(game.minimums.0, amount),
                Color::Green => game.minimums.1 = std::cmp::max(game.minimums.1, amount),
                Color::Blue => game.minimums.2 = std::cmp::max(game.minimums.2, amount),
            }
        });

        let power = game.minimums.0 * game.minimums.1 * game.minimums.2;
        sum_power += power;
    });

    println!("Sum power: {}", sum_power);
}
