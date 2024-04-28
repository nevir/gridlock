use crate::traits::*;

pub struct SquareCoordinate {
    pub x: i32,
    pub y: i32,
}

pub struct SquareCell {
    pub coordinate: SquareCoordinate,
}

impl Cell<SquareCoordinate> for SquareCell {
    fn get_coordinate(&self) -> &SquareCoordinate {
        return &self.coordinate;
    }
}

pub struct SquareGrid {}

impl Grid for SquareGrid {
    type Cell = SquareCell;
    type CellCoordinate = SquareCoordinate;

    fn get_cell_at_coordinate(&self, _coordinate: &Self::CellCoordinate) -> Option<&Self::Cell> {
        return None;
    }
}

impl PlanarGrid for SquareGrid {
    fn get_cell_center(&self, cell: &Self::Cell) -> PlanarPosition {
        return PlanarPosition {
            x: (cell.coordinate.x as f32) + 0.5,
            y: (cell.coordinate.y as f32) + 0.5,
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
