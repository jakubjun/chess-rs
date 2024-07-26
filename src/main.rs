use std::{collections::HashMap, io::stdin};

fn read_num() -> u32 {
    let i = stdin();
    let mut buf = String::new();
    i.read_line(&mut buf).unwrap();
    buf.trim_end().parse().unwrap()
}

struct Game {
    board: HashMap<Coords, Figure>,
    at_turn: Color,
    turns: u32,
    winner: Option<Color>,
}

impl Game {
    fn new() -> Self {
        let mut board: HashMap<Coords, Figure> = HashMap::new();

        for i in 0..8 {
            board.insert(
                (1, i),
                Figure {
                    color: Color::White,
                    variant: FigureVariant::Pawn,
                },
            );
        }

        Game {
            board,
            at_turn: Color::White,
            turns: 0,
            winner: None,
        }
    }

    fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                match self.board.get(&(i, j)) {
                    Some(figure) => print!(
                        "{}{} ",
                        figure.get_color_symbol(),
                        figure.variant.get_symbol()
                    ),
                    None => print!("   "),
                }
            }

            println!("");
        }
    }

    fn turn(&mut self) -> Result<(), ()> {
        let sx = read_num();
        let sy = read_num();

        let tx = read_num();
        let ty = read_num();

        let selected = self.board.get(&(sx, sy)).ok_or(())?;
        let targeted = self.board.get(&(tx, ty));

        if let Some(x) = targeted {
            if selected.can_take(&(sx, sy), self, &(tx, ty)) {
                let selected = self.board.remove(&(sx, sy)).ok_or(())?;
                self.board.insert((tx, ty), selected);
            }
        } else if selected.can_go(&(sx, sy), self, &(tx, ty)) {
            let selected = self.board.remove(&(sx, sy)).ok_or(())?;
            self.board.insert((tx, ty), selected);
        };
        Ok(())
    }
}
type Coords = (u32, u32);

enum Color {
    White,
    Black,
}

struct Figure {
    color: Color,
    variant: FigureVariant,
}

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
        match self.variant {
            FigureVariant::Pawn => {
                let y_diff: i8 = match self.color {
                    Color::White => -1,
                    Color::Black => 1,
                };

                field.0 == target.0 && (target.1 as i8 - field.1 as i8) == y_diff
            }
            _ => false,
        }
        // match self.variant {
        //     FigureVariant::Pawn => target
        //     _ => {}
        // }
    }

    fn can_take(&self, field: &Coords, game: &Game, target: &Coords) -> bool {
        match self.variant {
            FigureVariant::Pawn => {
                let y_diff: i8 = match self.color {
                    Color::White => -1,
                    Color::Black => 1,
                };

                field.0.abs_diff(target.0) == 1 && (target.1 as i8 - field.1 as i8) == y_diff
            }
            _ => false,
        }
    }

    fn get_color_symbol(&self) -> char {
        match self.color {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl FigureVariant {
    fn get_symbol(&self) -> char {
        match self {
            Self::Rook => 'r',
            Self::Pawn => 'p',
            Self::Knight => 'k',
            Self::Queen => 'q',
            Self::Bishop => 'b',
            Self::King => 'K',
        }
    }
}

fn main() -> Result<(), ()> {
    let mut g = Game::new();
    g.print();

    loop {
        g.turn();
    }

    Ok(())
}
