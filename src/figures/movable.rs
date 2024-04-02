use super::figures;
use super::utils;

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

impl Movable for figures::Tower {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        't'
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
    fn can_go_through(&self) -> bool {
        false
    }
}

impl Movable for figures::Bishop {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        'b'
    }
    fn get_color(&self) -> utils::Color {
        self.color
    }
    fn can_go_through(&self) -> bool {
        false
    }
}

impl Movable for figures::King {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        'K'
    }
    fn get_color(&self) -> utils::Color {
        self.color
    }
    fn can_go_through(&self) -> bool {
        false
    }
}

impl Movable for figures::Queen {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        'q'
    }
    fn get_color(&self) -> utils::Color {
        self.color
    }
    fn can_go_through(&self) -> bool {
        false
    }
}

impl Movable for figures::Knight {
    fn can_go_to_coords(&self, from: utils::Coords, target: utils::Coords) -> bool {
        from.0 == target.0 || from.1 == target.1
    }
    fn get_symbol(&self) -> char {
        'k'
    }
    fn get_color(&self) -> utils::Color {
        self.color
    }
    fn can_go_through(&self) -> bool {
        false
    }
}
