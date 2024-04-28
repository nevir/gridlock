use crate::traits::*;

pub struct HexCoordinate {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

pub struct HexCell {
    pub coordinate: HexCoordinate,
}

impl Cell<HexCoordinate> for HexCell {
    fn get_coordinate(&self) -> &HexCoordinate {
        return &self.coordinate;
    }
}

pub struct HexGrid {}

impl Grid for HexGrid {
    type Cell = HexCell;
    type CellCoordinate = HexCoordinate;

    fn get_cell_at_coordinate(&self, _coordinate: &Self::CellCoordinate) -> Option<&Self::Cell> {
        return None;
    }
}

const SQRT_3: f32 = 1.73205077648162841796875;
const SQRT_3_HALF: f32 = SQRT_3 / 2.0;

impl PlanarGrid for HexGrid {
    fn get_cell_center(&self, cell: &Self::Cell) -> PlanarPosition {
        return PlanarPosition {
            x: SQRT_3 * (cell.coordinate.q as f32) + SQRT_3_HALF,
            y: 1.5 * (cell.coordinate.r as f32),
        };
    }

    fn get_cell_at_position(&self, _position: &PlanarPosition) -> Option<&Self::Cell> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

// use crate::grid::*;

// pub struct HexCoordinate {

// }

// impl Coordinate for HexCoordinate {}

// pub struct HexCell {}

// impl Cell for HexCell {
//     type Coordinate = HexCoordinate;
// }

// pub struct HexGrid {}

// impl Grid for HexGrid {
//     type Cell = HexCell;
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
