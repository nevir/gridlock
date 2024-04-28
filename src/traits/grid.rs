pub trait Cell<TCoordinate> {
    fn get_coordinate(&self) -> &TCoordinate;
}

pub trait Grid {
    type Cell: Cell<Self::CellCoordinate>;
    type CellCoordinate;

    fn get_cell_at_coordinate(&self, coordinate: &Self::CellCoordinate) -> Option<&Self::Cell>;
}
