use crate::Grid;

pub struct PlanarPosition {
    pub x: f32,
    pub y: f32,
}

pub trait PlanarGrid: Grid {
    fn get_cell_center(&self, cell: &Self::Cell) -> PlanarPosition;

    fn get_cell_at_position(&self, position: &PlanarPosition) -> Option<&Self::Cell>;
}
