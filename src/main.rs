use std::fmt::{Display, Formatter, Result};
use std::io::Write;

pub trait Colored {
    fn get_color(&self) -> Color;

}

pub trait Movable {
    fn can_go(&self, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> bool;
}

impl Movable for Rook {
    fn can_go(&self, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> bool {
        match (from_x, from_y, to_x, to_y) {
            (a, b, c, d) if (c == a) && (self.color == Color::Black) && (d == b+1) => true,
            (a, b, c, d) if (c == a) && (self.color == Color::White) && (d == b-1) => true,
            _ => false

        }
    }
}

pub trait Figure: Colored + Display {}

impl Figure for Rook {}
impl Figure for Tower {}

impl Colored for Rook {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl Colored for Tower {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl Display for Rook {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "r")
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "t")
    }
}

struct Rook {
    color:  Color
}

#[test]
fn rook_movement() {
    let b = Rook {color: Color::Black};
    let w = Rook {color: Color::White};

    assert_eq!(b.can_go(0, 0, 0, 1), true);
    assert_eq!(w.can_go(0, 7, 0, 6), true);

    assert_eq!(b.can_go(0, 0, 1, 1), true);
}

struct Tower {
    color: Color
}

pub struct Board {
    positions: [[Option<Box<dyn Figure>>; 8]; 8]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White
}

impl Board {
    pub fn new() -> Self {
        Self {positions: [
            [Some(Box::new(Tower { color: Color::Black })),None,None,None,None,None,None,None],
            [Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black })),
             Some(Box::new(Rook { color: Color::Black }))
            ],
            [None,None,None,None,None,None,None,None],
            [None,None,None,None,None,None,None,None],
            [None,None,None,None,None,None,None,None],
            [None,None,None,None,None,None,None,None],
            [None,None,None,None,None,None,None,None],
            [None,None,None,None,None,None,None,None],
        ] }
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<Box<dyn Figure>> {
        &self.positions[x][y]
    }
}


#[test]
fn create_board() {

    let b = Board::new();
    let z = match b.get(1,1) {
        Some(_x) => 'a',
        None => 'n'
    };
    assert_eq!(z, 'a');

    let z = match b.get(0,0) {
        Some(_x) => 'a',
        None => 'n'
    };
    assert_eq!(z, 'a');
}

fn main() -> std::io::Result<()> {
    let mut w = Vec::new();

    write!(&mut w, "test")?;

    // let _rk = Rook { coords: (0,0) };
    Ok(())
}
