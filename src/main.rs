use std::{collections::HashMap, fmt::Display, io::stdin};

use colored::{Colorize, CustomColor};

fn read_algebraic() -> Result<(Coords, Coords), ()> {
    let i = stdin();
    let mut buf = String::new();
    i.read_line(&mut buf).or(Err(()))?;
    let from_x: char = buf[0..1].parse().or(Err(()))?;
    let from_y: u8 = buf[1..2].parse().or(Err(()))?;
    let to_x: char = buf[2..3].parse().or(Err(()))?;
    let to_y: u8 = buf[3..4].parse().or(Err(()))?;
    Ok(((from_x, from_y), (to_x, to_y)))
}

struct Game {
    board: HashMap<Coords, Figure>,
    at_turn: FigureColor,
    turns: u32,
    winner: Option<FigureColor>,
    check: Option<FigureColor>,
}

impl Display for FigureColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "{}", " white ".black().on_white()),
            Self::Black => write!(f, "{}", " black ".white().on_black()),
        }?;
        Ok(())
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_symbol().to_string().black())?;
        Ok(())
    }
}

impl Game {
    fn new() -> Self {
        const default_setup: &[&str; 8] = &[
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

    fn from_str_arr(setup: &[&str; 8]) -> Self {
        let mut board: HashMap<Coords, Figure> = HashMap::new();

        for (i, row) in setup.iter().enumerate() {
            // let mut row_iter = row.chars();
            //
            // while let Some(pair) = row_iter.next() {}
            //
            // row_iter.nth(2);
            let mut curr = &row[0..];
            let mut j = 0;

            while curr.len() >= 2 {
                let fig = Figure::from_str(&curr[..2]);

                if let Some(fig) = fig {
                    let letter = ('a' as u8 + j) as char;
                    let number = 8 - i as u8;
                    dbg!((letter, number));
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

    fn turn(&mut self) -> Result<(), ()> {
        println!("{}", "----------------------------".cyan());
        loop {
            println!("{}'s turn", self.at_turn);
            println!("{}", self);
            println!("input algebraic notation, eg 'a1a2': ");
            let ((sx, sy), (tx, ty)) = read_algebraic()?;

            let selected = self.board.get(&(sx, sy)).ok_or(())?;

            if selected.color != self.at_turn {
                println!("{}", "err: cannot move opponent's figure".red());
                continue;
            }

            if selected.can_go(&(sx, sy), self, &(tx, ty)) {
                let selected = self.board.remove(&(sx, sy)).ok_or(())?;
                self.board.insert((tx, ty), selected);
                break;
            } else {
                println!("{}", "err: this figure cannot go there".red());
            };
        }
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

    fn game_loop(&mut self) -> Result<(), ()> {
        while self.winner.is_none() {
            self.turn()?;
        }
        Ok(())
    }
}
type Coords = (char, u8);

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

fn get_bg_color(coords: Coords) -> CustomColor {
    let light_bg = CustomColor::new(204, 183, 174);
    let dark_bg = CustomColor::new(112, 102, 119);
    match coords {
        (y, x) if x % 2 == y as u8 % 2 => light_bg,
        _ => dark_bg,
    }
}

#[derive(Debug, PartialEq)]
enum FigureColor {
    White,
    Black,
}

#[derive(Debug)]
struct Figure {
    color: FigureColor,
    variant: FigureVariant,
}

#[derive(Debug)]
enum FigureVariant {
    Rook,
    Pawn,
    Knight,
    Queen,
    Bishop,
    King,
}

impl Figure {
    fn can_go(&self, field: &Coords, game: &Game, target: &Coords) -> bool {
        if field == target {
            println!("select different field than the current one");
            return false;
        }

        if let Some(target) = game.board.get(target) {
            if target.color == self.color {
                println!("cant take your figure");
                return false;
            }
        }

        match self.variant {
            FigureVariant::Pawn => {
                let y_diff: i8 = match self.color {
                    FigureColor::White => -1,
                    FigureColor::Black => 1,
                };

                let allow_double = matches!(
                    (&self.color, field.1),
                    (FigureColor::White, 7) | (FigureColor::Black, 2)
                );

                let mut goes_straight =
                    field.0 == target.0 && ((target.1 as i8 - field.1 as i8) == y_diff);

                if allow_double {
                    goes_straight |=
                        field.0 == target.0 && ((target.1 as i8 - field.1 as i8) == y_diff * 2);
                }

                // TODO this needs to check if the 'across' field is occupied by an opponent
                let goes_across = (field.0 as i8).abs_diff(target.0 as i8) == 1
                    && (target.1 as i8 - field.1 as i8) == y_diff;

                goes_straight || goes_across
            }
            FigureVariant::Rook => {
                if target.1 == field.1 && target.0 != field.0 {
                    let range = if field.0 > target.0 {
                        target.0 as u8..field.0 as u8 - 1
                    } else {
                        field.0 as u8 + 1..target.0 as u8
                    };
                    for i in range {
                        if game
                            .board
                            .get(&(('a' as u8 + i) as char, field.1))
                            .is_some()
                        {
                            return false;
                        }
                    }
                    true
                } else if target.1 != field.1 && target.0 == field.0 {
                    let range = if field.1 > target.1 {
                        target.1..field.1 - 1
                    } else {
                        field.1 + 1..target.1
                    };
                    for i in range {
                        if game.board.get(&(field.0, i)).is_some() {
                            return false;
                        }
                    }
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }
    fn get_symbol(&self) -> char {
        match (&self.color, &self.variant) {
            (FigureColor::White, FigureVariant::Rook) => '♖',
            (FigureColor::Black, FigureVariant::Rook) => '♜',
            (FigureColor::White, FigureVariant::Pawn) => '♙',
            (FigureColor::Black, FigureVariant::Pawn) => '♟',
            (FigureColor::White, FigureVariant::Knight) => '♘',
            (FigureColor::Black, FigureVariant::Knight) => '♞',
            (FigureColor::White, FigureVariant::Queen) => '♕',
            (FigureColor::Black, FigureVariant::Queen) => '♛',
            (FigureColor::White, FigureVariant::Bishop) => '♗',
            (FigureColor::Black, FigureVariant::Bishop) => '♝',
            (FigureColor::White, FigureVariant::King) => '♔',
            (FigureColor::Black, FigureVariant::King) => '♚',
        }
    }

    fn from_str(code: &str) -> Option<Self> {
        let color_code = code.chars().nth(0).unwrap();
        let variant_code = code.chars().nth(1).unwrap();

        let color = match color_code {
            'b' => Some(FigureColor::Black),
            'w' => Some(FigureColor::White),
            ' ' => None,
            _ => panic!(),
        };

        let variant = match variant_code {
            'p' => Some(FigureVariant::Pawn),
            'r' => Some(FigureVariant::Rook),
            'k' => Some(FigureVariant::Knight),
            'b' => Some(FigureVariant::Bishop),
            'K' => Some(FigureVariant::King),
            'q' => Some(FigureVariant::Queen),
            ' ' => None,
            _ => panic!(),
        };

        if color.is_none() || variant.is_none() {
            None
        } else {
            Some(Figure {
                variant: variant?,
                color: color?,
            })
        }
    }
}

fn main() -> Result<(), ()> {
    let mut g = Game::new();
    g.game_loop()?;
    Ok(())
}
