use crate::grid::*;

pub struct HexCoordinate {
  pub q: u32,
  pub r: u32,
  pub s: u32,
}

impl Coordinate for HexCoordinate {}

pub struct HexCell {

}

impl Cell for HexCell {
  type Coordinate = HexCoordinate;
}

pub struct HexGrid {

}

impl Grid for HexGrid {
  type Cell = HexCell;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
