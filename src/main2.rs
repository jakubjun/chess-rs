// https://users.rust-lang.org/t/why-doesnt-rust-allow-fields-in-trait/52191/9
// https://stackoverflow.com/a/71961087
// https://stackoverflow.com/questions/63407245/what-is-an-idiomatic-way-to-have-multiple-structs-with-the-same-properties-in-ru
// https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel
// https://d3s.mff.cuni.cz/teaching/nprg073/lecture_4/

use colored::Colorize;
use std::io;

use crate::figures::utils::Coords;

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
    println!();
    let selected = selected.unwrap_or(figures::utils::Coords(10, 10));

    for (i, row) in b.squares.iter().enumerate() {
        print!(" {} ", i);
        for (j, col) in row.iter().enumerate() {
            let curr_field_selected = i as u8 == selected.0 && j as u8 == selected.1;

            let is_square_light = i % 2 == j % 2;

            let symbol = match col {
                Some(square) => square.get_symbol(),
                None => ' ',
            };

            // defaults
            let (default_fg, default_bg) = match is_square_light {
                true if curr_field_selected => {
                    (symbol.to_string().red().on_white(), " ".black().on_white())
                }
                true => (
                    symbol
                        .to_string()
                        .truecolor(0, 0, 0)
                        .on_truecolor(200, 200, 200),
                    " ".truecolor(0, 0, 0).on_truecolor(200, 200, 200),
                ),
                false if curr_field_selected => {
                    (symbol.to_string().red().on_black(), " ".red().on_black())
                }
                false => (
                    symbol
                        .to_string()
                        .truecolor(0, 0, 0)
                        .on_truecolor(150, 150, 150),
                    " ".truecolor(0, 0, 0).on_truecolor(150, 150, 150),
                ),
            };

            let (fg, bg) = match (col, curr_field_selected) {
                (Some(_), true) => match is_square_light {
                    true => (
                        symbol.to_string().red().on_truecolor(155, 0, 0),
                        " ".to_string().truecolor(155, 0, 0).on_truecolor(155, 0, 0),
                    ),
                    false => (
                        symbol.to_string().red().on_truecolor(160, 160, 160),
                        " ".to_string().black().on_truecolor(160, 160, 160),
                    ),
                },

                (Some(square), false) => match square.get_color() {
                    figures::utils::Color::Black => match is_square_light {
                        true => (
                            symbol
                                .to_string()
                                .truecolor(0, 0, 0)
                                .on_truecolor(200, 200, 200),
                            " ".to_string()
                                .truecolor(0, 0, 0)
                                .on_truecolor(200, 200, 200),
                        ),
                        false => (
                            symbol
                                .to_string()
                                .truecolor(0, 0, 0)
                                .on_truecolor(150, 150, 150),
                            " ".truecolor(0, 0, 0).on_truecolor(150, 150, 150),
                        ),
                    },
                    figures::utils::Color::White => match is_square_light {
                        true => (
                            symbol
                                .to_string()
                                .truecolor(255, 255, 255)
                                .on_truecolor(200, 200, 200),
                            " ".truecolor(255, 255, 255).on_truecolor(120, 120, 120),
                        ),
                        false => (
                            symbol.to_string().white().on_black(),
                            " ".to_string().black().on_black(),
                        ),
                    },
                },
                (None, _) => (default_fg, default_bg),
            };

            print!("{}{}{}", bg, fg, bg)
        }
        println!();
    }
}

fn move_figure(b: &Board, from: figures::utils::Coords, to: figures::utils::Coords) {}

fn main() {
    let mut b = Board {
        squares: [
            [
                Some(Box::new(figures::figures::Rook {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Knight {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Bishop {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Queen {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::King {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Bishop {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Knight {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Rook {
                    color: figures::utils::Color::Black,
                })),
            ],
            [
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::Black,
                })),
                Some(Box::new(figures::figures::Pawn {
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
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Pawn {
                    color: figures::utils::Color::White,
                })),
            ],
            [
                Some(Box::new(figures::figures::Rook {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Knight {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Bishop {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Queen {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::King {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Bishop {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Knight {
                    color: figures::utils::Color::White,
                })),
                Some(Box::new(figures::figures::Rook {
                    color: figures::utils::Color::White,
                })),
            ],
        ],
    };

    loop {
        print_board(&b, None);

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

        let from = Coords(vec[0], vec[1]);

        print_board(&b, Some(selection));

        println!("select a target");
        input.clear();

        io::stdin().read_line(&mut input).expect("err reading");

        let vec2 = input
            .split_whitespace()
            .map(|x| x.parse::<u8>().expect("parse error"))
            .collect::<Vec<u8>>();

        if vec2.len() < 2 {
            println!("pls input two numbers");
            continue;
        }

        let targ = Coords(vec2[0], vec2[1]);

        b.squares[targ.0 as usize][targ.1 as usize] =
            b.squares[from.0 as usize][from.1 as usize].take();
    }
}
