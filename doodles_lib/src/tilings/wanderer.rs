//! Implementation of the Wanderer (reflections) tiling.
//!
//! This tilling uses a square proto-tile which has a reflection (handedness) and an orientation.
//! For the purposes of this implementation both of these concepts are
//! represented as enums, [`WandererTile`] and [`WandererTileOrientation`] respectively.
use super::Divisible;
use nannou::geom::Rect;

#[derive(Debug)]
pub enum WandererTileOrientation {
    Left,
    Right,
    Top,
    Bottom,
}

/// Enumeration of the two reflections for each tile in the tiling.
///
/// Each variant has a [`Rect`] and a [`WandererTileOrientation`] associated with it.
/// The rect is used when a tile is drawn to the screen, while the orientation is used to construct
/// and place valid descendents of a given tile.
#[derive(Debug)]
pub enum WandererTile {
    LeftHanded(Rect, WandererTileOrientation),
    RightHanded(Rect, WandererTileOrientation),
}

impl Divisible for WandererTile {
    fn divide(&self) -> Vec<Self> {
        let rect = match *self {
            WandererTile::LeftHanded(rect, _) | WandererTile::RightHanded(rect, _) => rect,
        };

        let divided_rect = Rect::from_w_h(rect.w() / 2.0, rect.h() / 2.0);

        let (top_left, top_right, bottom_left, bottom_right) = (
            divided_rect.top_left_of(rect),
            divided_rect.top_right_of(rect),
            divided_rect.bottom_left_of(rect),
            divided_rect.bottom_right_of(rect),
        );

        match self {
            WandererTile::LeftHanded(_, tile_orientation) => match tile_orientation {
                WandererTileOrientation::Bottom => {
                    vec![
                        WandererTile::LeftHanded(top_left, WandererTileOrientation::Left),
                        WandererTile::LeftHanded(top_right, WandererTileOrientation::Bottom),
                        WandererTile::RightHanded(bottom_left, WandererTileOrientation::Right),
                        WandererTile::RightHanded(bottom_right, WandererTileOrientation::Bottom),
                    ]
                }
                WandererTileOrientation::Left => {
                    vec![
                        WandererTile::RightHanded(top_left, WandererTileOrientation::Bottom),
                        WandererTile::LeftHanded(top_right, WandererTileOrientation::Top),
                        WandererTile::RightHanded(bottom_left, WandererTileOrientation::Left),
                        WandererTile::LeftHanded(bottom_right, WandererTileOrientation::Left),
                    ]
                }
                WandererTileOrientation::Top => {
                    vec![
                        WandererTile::RightHanded(top_left, WandererTileOrientation::Top),
                        WandererTile::RightHanded(top_right, WandererTileOrientation::Left),
                        WandererTile::LeftHanded(bottom_left, WandererTileOrientation::Top),
                        WandererTile::LeftHanded(bottom_right, WandererTileOrientation::Right),
                    ]
                }
                WandererTileOrientation::Right => {
                    vec![
                        WandererTile::LeftHanded(top_left, WandererTileOrientation::Right),
                        WandererTile::RightHanded(top_right, WandererTileOrientation::Right),
                        WandererTile::LeftHanded(bottom_left, WandererTileOrientation::Bottom),
                        WandererTile::RightHanded(bottom_right, WandererTileOrientation::Top),
                    ]
                }
            },
            WandererTile::RightHanded(_, tile_orientation) => match tile_orientation {
                WandererTileOrientation::Bottom => {
                    vec![
                        WandererTile::RightHanded(top_left, WandererTileOrientation::Bottom),
                        WandererTile::RightHanded(top_right, WandererTileOrientation::Right),
                        WandererTile::LeftHanded(bottom_left, WandererTileOrientation::Bottom),
                        WandererTile::LeftHanded(bottom_right, WandererTileOrientation::Left),
                    ]
                }
                WandererTileOrientation::Left => {
                    vec![
                        WandererTile::LeftHanded(top_left, WandererTileOrientation::Left),
                        WandererTile::RightHanded(top_right, WandererTileOrientation::Left),
                        WandererTile::LeftHanded(bottom_left, WandererTileOrientation::Top),
                        WandererTile::RightHanded(bottom_right, WandererTileOrientation::Bottom),
                    ]
                }
                WandererTileOrientation::Top => {
                    vec![
                        WandererTile::LeftHanded(top_left, WandererTileOrientation::Right),
                        WandererTile::LeftHanded(top_right, WandererTileOrientation::Top),
                        WandererTile::RightHanded(bottom_left, WandererTileOrientation::Left),
                        WandererTile::RightHanded(bottom_right, WandererTileOrientation::Top),
                    ]
                }
                WandererTileOrientation::Right => {
                    vec![
                        WandererTile::RightHanded(top_left, WandererTileOrientation::Top),
                        WandererTile::LeftHanded(top_right, WandererTileOrientation::Bottom),
                        WandererTile::RightHanded(bottom_left, WandererTileOrientation::Right),
                        WandererTile::LeftHanded(bottom_right, WandererTileOrientation::Right),
                    ]
                }
            },
        }
    }
}
