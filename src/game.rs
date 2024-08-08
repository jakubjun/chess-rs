use std::{collections::HashMap, fmt::Display, io::stdin};

use colored::{Colorize, CustomColor};

use crate::figure::{Figure, FigureColor};

pub type BoardSetup = &'static [&'static str; 8];
pub type Coords = (char, u8);

fn get_bg_color(coords: Coords) -> CustomColor {
    let light_bg = CustomColor::new(204, 183, 174);
    let dark_bg = CustomColor::new(112, 102, 119);
    match coords {
        (y, x) if x % 2 == y as u8 % 2 => light_bg,
        _ => dark_bg,
    }
}

fn read_algebraic() -> Result<(Coords, Coords), &'static str> {
    let i = stdin();
    let mut buf = String::new();
    i.read_line(&mut buf).or(Err("unable to read input"))?;
    let from_x: char = buf[0..1]
        .parse()
        .or(Err("unable to parse 'from_x', it has to be a letter"))?;
    let from_y: u8 = buf[1..2]
        .parse()
        .or(Err("unable to parse 'from_y', it has to be a number"))?;
    let to_x: char = buf[2..3]
        .parse()
        .or(Err("unable to parse 'to_x', it has to be a letter"))?;
    let to_y: u8 = buf[3..4]
        .parse()
        .or(Err("unable to parse 'to_y', it has to be a number"))?;
    Ok(((from_x, from_y), (to_x, to_y)))
}

#[allow(dead_code)]
enum GameState {
    Check(FigureColor),
    Checkmate(FigureColor),
    Stalemate(FigureColor),
    Play,
}

pub struct Game {
    pub board: HashMap<Coords, Figure>,
    at_turn: FigureColor,
    #[allow(dead_code)]
    turns: u32,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        const DEFAULT_SETUP: BoardSetup = &[
            "brbkbbbqbKbbbkbr",
            "bpbpbpbpbpbpbpbp",
            "                ",
            "                ",
            "                ",
            "                ",
            "wpwpwpwpwpwpwpwp",
            "wrwkwbwqwKwbwkwr",
        ];
        Self::from_str_arr(DEFAULT_SETUP)
    }

    fn from_str_arr(setup: BoardSetup) -> Self {
        let mut board: HashMap<Coords, Figure> = HashMap::new();

        for (rank_idx, rank) in setup.iter().enumerate() {
            let mut curr = &rank[0..];
            let mut file_cnt = 0;

            while curr.len() >= 2 {
                let fig = Figure::from_str(&curr[..2]);

                if let Some(fig) = fig {
                    let letter = char::from_u32('a' as u32 + file_cnt).unwrap();
                    let number = 8 - rank_idx as u8;
                    board.insert((letter, number), fig);
                }

                file_cnt += 1;
                curr = &curr[2..];
            }
        }

        Game {
            board,
            at_turn: FigureColor::White,
            turns: 0,
            state: GameState::Play,
        }
    }

    fn turn(&mut self) -> Result<(), &str> {
        println!("{}", "----------------------------".cyan());
        println!("{}'s turn", self.at_turn);
        println!("{}", self);
        println!("input '{{from}}{{to}}', eg 'a1a2': ");
        let ((from_x, from_y), (to_x, to_y)) = read_algebraic()?;

        let selected_figure = self
            .board
            .remove(&(from_x, from_y))
            .ok_or("theres no figure on that field")?;

        if selected_figure.color != self.at_turn {
            self.board.insert((from_x, from_y), selected_figure);
            return Err("cannot move opponent's figure");
        }

        match selected_figure.can_go(&(from_x, from_y), self, &(to_x, to_y)) {
            Ok(_) => {
                self.board.insert((to_x, to_y), selected_figure);
                Ok(())
            }
            Err(e) => {
                self.board.insert((from_x, from_y), selected_figure);
                Err(e)
            }
        }?;

        self.switch_sides();
        Ok(())
    }

    fn switch_sides(&mut self) {
        if self.at_turn == FigureColor::Black {
            self.at_turn = FigureColor::White
        } else {
            self.at_turn = FigureColor::Black
        }
    }

    pub fn game_loop(&mut self) {
        while matches!(self.state, GameState::Play | GameState::Check(_)) {
            match self.turn() {
                Ok(_) => (),
                Err(e) => println!("{}", format!("err: {}", e).red()),
            };
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for file_letter in 'a'..'i' {
            write!(f, " {} ", file_letter)?;
        }
        writeln!(f)?;

        for rank_idx in (1..9).rev() {
            write!(f, "{} ", rank_idx)?;

            for file_letter in 'a'..'i' {
                match self.board.get(&(file_letter, rank_idx)) {
                    Some(figure) => {
                        let symbol = format!(" {} ", figure)
                            .on_custom_color(get_bg_color((file_letter, rank_idx)));
                        write!(f, "{}", symbol)?;
                    }
                    None => {
                        let symbol = "   ".on_custom_color(get_bg_color((file_letter, rank_idx)));
                        write!(f, "{}", symbol)?;
                    }
                }
            }
            write!(f, " {}", rank_idx)?;
            writeln!(f)?;
        }
        write!(f, "  ")?;

        for file_letter in 'a'..'i' {
            write!(f, " {} ", file_letter)?;
        }

        Ok(())
    }
}
