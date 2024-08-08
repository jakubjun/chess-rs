use colored::Colorize;
use std::fmt::Display;

use crate::game::{Coords, Game};

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

#[derive(Debug, PartialEq)]
pub enum FigureColor {
    White,
    Black,
}

#[derive(Debug)]
pub struct Figure {
    pub color: FigureColor,
    variant: FigureVariant,
}

#[derive(Debug)]
pub enum FigureVariant {
    Rook,
    Pawn,
    Knight,
    Queen,
    Bishop,
    King,
}

impl Figure {
    pub fn can_go(&self, field: &Coords, game: &Game, target: &Coords) -> Result<(), &'static str> {
        if field == target {
            return Err("select different field than the current one");
        }

        if let Some(target) = game.board.get(target) {
            if target.color == self.color {
                return Err("cant take your figure");
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
                let takes_across = (field.0 as i8).abs_diff(target.0 as i8) == 1
                    && (target.1 as i8 - field.1 as i8) == y_diff;

                if goes_straight || takes_across {
                    Ok(())
                } else {
                    Err("the figure cannot go there")
                }
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
                            return Err("something is in the way");
                        }
                    }
                    Ok(())
                } else if target.1 != field.1 && target.0 == field.0 {
                    let range = if field.1 > target.1 {
                        target.1..field.1 - 1
                    } else {
                        field.1 + 1..target.1
                    };
                    for i in range {
                        if game.board.get(&(field.0, i)).is_some() {
                            return Err("something is in the way");
                        }
                    }
                    Ok(())
                } else {
                    Err("the figure cannot go there")
                }
            }
            _ => unimplemented!(),
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

    pub fn from_str(code: &str) -> Option<Self> {
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
