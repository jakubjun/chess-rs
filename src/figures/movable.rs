use super::utils;
use super::figures;
use std::fmt;

pub trait Movable {
    fn can_go_to_coords(&self, from: utils::Coords, to: utils::Coords) -> bool {
        unimplemented!();
    }

    fn get_available_coords(&self, from: utils::Coords) -> Vec<utils::Coords> {
        unimplemented!();
    }
    fn can_go_through(&self) -> bool {
        unimplemented!();
    }
    fn get_symbol(&self) -> char {
        unimplemented!();
    }
    fn get_color(&self) -> utils::Color {
        unimplemented!();
    }
}

impl fmt::Display for dyn Movable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}



impl Movable for figures::Tower {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        'T'
    }
    fn get_color(&self) -> utils::Color {
        self.color
    }
}

impl Movable for figures::Pawn {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        'p'
    }
    fn get_color(&self) -> utils::Color {
        self.color
    }
}

// impl fmt::Display for Figure {

// }

impl fmt::Display for figures::Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "T")
    }
}

