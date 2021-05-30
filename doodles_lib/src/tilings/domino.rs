//! Implementation of the Domino (Table) tiling.
//!
//! This tiling divides two proto-tiles, a horizontal and a vertical one, in four.
//! The subdivision process varies for each type of tile, the difference being in the division
//! proportions and the placing of its children.
use super::{Divisible, Rectangular};
use nannou::geom::Rect;

/// Enumeration of the tile variants used to construct a domino tiling.
///
/// Each variant has a [`Rect`] as an associated value, this rect is used when the tile is drawn to
/// the screen.
#[derive(Debug)]
pub enum DominoTile {
    Horizontal(Rect),
    Vertical(Rect),
}

impl Divisible for DominoTile {
    fn divide(&self) -> Vec<Self> {
        match *self {
            DominoTile::Horizontal(rect) => {
                let h_rect = Rect::from_w_h(rect.w() / 2.0, rect.h() / 2.0);
                let v_rect = Rect::from_w_h(rect.w() / 4.0, rect.h());

                vec![
                    DominoTile::Vertical(v_rect.top_left_of(rect)),
                    DominoTile::Horizontal(h_rect.mid_top_of(rect)),
                    DominoTile::Horizontal(h_rect.mid_bottom_of(rect)),
                    DominoTile::Vertical(v_rect.top_right_of(rect)),
                ]
            }
            DominoTile::Vertical(rect) => {
                let h_rect = Rect::from_w_h(rect.w(), rect.h() / 4.0);
                let v_rect = Rect::from_w_h(rect.w() / 2.0, rect.h() / 2.0);

                vec![
                    DominoTile::Horizontal(h_rect.top_left_of(rect)),
                    DominoTile::Vertical(v_rect.mid_left_of(rect)),
                    DominoTile::Vertical(v_rect.mid_right_of(rect)),
                    DominoTile::Horizontal(h_rect.bottom_left_of(rect)),
                ]
            }
        }
    }
}

impl Rectangular for DominoTile {
    fn rect(&self) -> &Rect {
        match self {
            DominoTile::Horizontal(rect) | DominoTile::Vertical(rect) => rect,
        }
    }
}
