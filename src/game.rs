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

pub struct Game {
    pub board: HashMap<Coords, Figure>,
    at_turn: FigureColor,
    turns: u32,
    winner: Option<FigureColor>,
    check: Option<FigureColor>,
}

impl Game {
    pub fn new() -> Self {
        const default_setup: BoardSetup = &[
            "wrwkwbwqwKwbwkwr",
            "wpwpwpwpwpwpwpwp",
            "                ",
            "                ",
            "                ",
            "                ",
            "bpbpbpbpbpbpbpbp",
            "brbkbbbqbKbbbkbr",
        ];
        Self::from_str_arr(default_setup)
    }

    fn from_str_arr(setup: BoardSetup) -> Self {
        let mut board: HashMap<Coords, Figure> = HashMap::new();

        for (i, row) in setup.iter().enumerate() {
            let mut curr = &row[0..];
            let mut j = 0;

            while curr.len() >= 2 {
                let fig = Figure::from_str(&curr[..2]);

                if let Some(fig) = fig {
                    let letter = ('a' as u8 + j) as char;
                    let number = 8 - i as u8;
                    board.insert((letter, number), fig);
                }

                j += 1;
                curr = &curr[2..];
            }
        }

        Game {
            board,
            at_turn: FigureColor::White,
            winner: None,
            check: None,
            turns: 0,
        }
    }

    fn turn(&mut self) -> Result<(), &str> {
        println!("{}", "----------------------------".cyan());
        println!("{}'s turn", self.at_turn);
        println!("{}", self);
        println!("input algebraic notation, eg 'a1a2': ");
        let ((sx, sy), (tx, ty)) = read_algebraic()?;

        let selected = self
            .board
            .remove(&(sx, sy))
            .ok_or("theres no figure on that field")?;

        if selected.color != self.at_turn {
            return Err("cannot move opponent's figure");
        }

        match selected.can_go(&(sx, sy), self, &(tx, ty)) {
            Ok(_) => {
                self.board.insert((tx, ty), selected);
                Ok(())
            }
            Err(e) => {
                self.board.insert((sx, sy), selected);
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

    fn check(&mut self) {
        todo!()
    }

    fn checkmate(&mut self) {
        todo!()
    }

    pub fn game_loop(&mut self) {
        while self.winner.is_none() {
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
        for j in 'a'..'i' {
            write!(f, " {} ", j)?;
        }
        writeln!(f)?;
        for i in (1..9).rev() {
            write!(f, "{} ", i)?;
            for j in 'a'..'i' {
                match self.board.get(&(j, i)) {
                    Some(figure) => {
                        let symbol = format!(" {} ", figure).on_custom_color(get_bg_color((j, i)));
                        write!(f, "{}", symbol)?;
                    }
                    None => {
                        let symbol = "   ".on_custom_color(get_bg_color((j, i)));
                        write!(f, "{}", symbol)?;
                    }
                }
            }
            write!(f, " {}", i)?;
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for j in 'a'..'i' {
            write!(f, " {} ", j)?;
        }
        Ok(())
    }
}
