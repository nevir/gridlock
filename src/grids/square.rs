use std::marker::PhantomData;

use crate::grid::*;
use num::Integer;

// Coordinates

pub struct SquareCoordinate<TComponent: Integer = u32> {
  pub x: TComponent,
  pub y: TComponent,
}

impl<TComponent: Integer> Coordinate for SquareCoordinate<TComponent> {}

// Cells

pub struct SquareCell<TComponent: Integer = u32> {
  component_type: PhantomData<TComponent>,
}

impl<TComponent: Integer> Cell for SquareCell<TComponent> {
  type Coordinate = SquareCoordinate<TComponent>;
}

// Grid

pub struct SquareGrid<TComponent: Integer = u32> {
  component_type: PhantomData<TComponent>,
}

impl<TComponent: Integer> SquareGrid<TComponent> {
  pub fn new() -> Self {
    Self {
      component_type: PhantomData,
    }
  }
}

impl<TComponent: Integer> Grid<SquareCoordinate<TComponent>> for SquareGrid<TComponent> {
  type Cell = SquareCell<TComponent>;

  fn get_cell(&self, _coordinate: &SquareCoordinate<TComponent>) -> Option<Self::Cell> {
    return None;
  }
}

#[cfg(test)]
mod tests {
    use crate::square::SquareGrid;

  #[test]
  fn it_works() {
    let grid = SquareGrid::<u8>::new();

  }
}
