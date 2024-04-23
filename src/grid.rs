pub trait Coordinate {
}

pub trait Cell {
  type Coordinate: Coordinate;
}

pub trait Grid<TCoordinate: Coordinate> {
  type Cell: Cell<Coordinate = TCoordinate>;
  fn get_cell(&self, coordinate: &TCoordinate) -> Option<Self::Cell>;
}
