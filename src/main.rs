// https://users.rust-lang.org/t/why-doesnt-rust-allow-fields-in-trait/52191/9
// https://stackoverflow.com/a/71961087
// https://stackoverflow.com/questions/63407245/what-is-an-idiomatic-way-to-have-multiple-structs-with-the-same-properties-in-ru
// https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel
// https://d3s.mff.cuni.cz/teaching/nprg073/lecture_4/

use colored::Colorize;
use std::io;


pub mod figures {
    pub mod figures;
    pub mod movable;
    pub mod utils;
}


struct Board {
    squares: [[Option<Box<dyn figures::movable::Movable>>; 8]; 8],
}

fn print_board(b: &Board, selected: Option<figures::utils::Coords>) {
    print!("   ");
    for (i, _) in b.squares.iter().enumerate() {
        print!(" {} ", i)
    }
    print!("\n");
    let selected = selected.unwrap_or(figures::utils::Coords(10, 10));

    for (i, row) in b.squares.iter().enumerate() {
        print!(" {} ", i);
        for (j, col) in row.iter().enumerate() {
            match col {
                Some(square) if i as u8 == selected.0 && j as u8 == selected.1 => print!(
                    "{}{}{}",
                    " ".black().on_purple(),
                    square.get_symbol().to_string().black().on_purple(),
                    " ".black().on_purple()
                ),
                Some(square) => match (*square).get_color() {
                    figures::utils::Color::Black => print!(
                        "{}{}{}",
                        " ".white().on_bright_black(),
                        square.get_symbol().to_string().white().on_bright_black(),
                        " ".white().on_bright_black()
                    ),
                    figures::utils::Color::White => print!(
                        "{}{}{}",
                        " ".black().on_white(),
                        square.get_symbol().to_string().black().on_white(),
                        " ".black().on_white()
                    ),
                },
                None => print!("{}", " _ ".black().on_bright_yellow()),
            }
        }
        print!("\n");
    }
}

fn move_figure(b: &Board, from: figures::utils::Coords, to: figures::utils::Coords) {}

fn main() {
    let b = Board {
        squares: [
            [
                Some(Box::new(figures::figures::Tower {
                    color: figures::utils::Color::Black,
                })),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Box::new(figures::figures::Tower {
                    color: figures::utils::Color::Black,
                })),
            ],
            [
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Box::new(figures::figures::Tower {
                    color: figures::utils::Color::Black,
                })),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
            ],
            [
                Some(Box::new(figures::figures::Tower {
                    color: figures::utils::Color::White,
                })),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Box::new(figures::figures::Tower {
                    color: figures::utils::Color::White,
                })),
            ],
        ],
    };

    print_board(&b, None);

    loop {
        // b.squares[0][2] = b.squares[1][0].take();

        let mut input = String::new();

        println!("select a figure");
        io::stdin().read_line(&mut input).expect("err reading");

        let vec = input
            .split_whitespace()
            .map(|x| x.parse::<u8>().expect("parse error"))
            .collect::<Vec<u8>>();

        let selection = figures::utils::Coords(vec[0], vec[1]);

        if vec.len() < 2 {
            println!("pls input two numbers");
            continue;
        }

        print_board(&b, Some(selection));

        // println!("{}", x);
    }

    // print_board(&b)
}
