use nannou::geom::Rect;

pub trait Divide {
    fn divide(&self) -> Vec<Self>
    where
        Self: Sized;
}

pub struct TileData {
    pub rect: Rect,
}

impl TileData {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

pub enum DominoTile {
    Horizontal(TileData),
    Vertical(TileData),
}

impl Divide for DominoTile {
    fn divide(&self) -> Vec<Self> {
        match self {
            DominoTile::Horizontal(tile_data) => {
                let h_rect = Rect::from_w_h(tile_data.rect.w() / 2.0, tile_data.rect.h() / 2.0);
                let v_rect = Rect::from_w_h(tile_data.rect.w() / 4.0, tile_data.rect.h());

                vec![
                    DominoTile::Vertical(TileData::new(
                        v_rect.top_left_of(tile_data.rect).pad_right(20.0),
                    )),
                    DominoTile::Horizontal(TileData::new(
                        h_rect.mid_top_of(tile_data.rect).pad_bottom(10.0),
                    )),
                    DominoTile::Horizontal(TileData::new(
                        h_rect.mid_bottom_of(tile_data.rect).pad_top(10.0),
                    )),
                    DominoTile::Vertical(TileData::new(
                        v_rect.top_right_of(tile_data.rect).pad_left(20.0),
                    )),
                ]
            }
            DominoTile::Vertical(tile_data) => {
                let h_rect = Rect::from_w_h(tile_data.rect.w(), tile_data.rect.h() / 4.0);
                let v_rect = Rect::from_w_h(tile_data.rect.w() / 2.0, tile_data.rect.h() / 2.0);

                vec![
                    DominoTile::Horizontal(TileData::new(
                        h_rect.top_left_of(tile_data.rect).pad_bottom(20.0),
                    )),
                    DominoTile::Vertical(TileData::new(
                        v_rect.mid_left_of(tile_data.rect).pad_right(10.0),
                    )),
                    DominoTile::Vertical(TileData::new(
                        v_rect.mid_right_of(tile_data.rect).pad_left(10.0),
                    )),
                    DominoTile::Horizontal(TileData::new(
                        h_rect.bottom_left_of(tile_data.rect).pad_top(20.0),
                    )),
                ]
            }
        }
    }
}

pub enum WandererTileOrientation {
    Left,
    Right,
    Top,
    Bottom,
}

pub enum WandererTile {
    LeftHanded(TileData, WandererTileOrientation),
    RightHanded(TileData, WandererTileOrientation),
}

impl Divide for WandererTile {
    fn divide(&self) -> Vec<Self> {
        let tile_data = match self {
            WandererTile::LeftHanded(tile_data, _) | WandererTile::RightHanded(tile_data, _) => {
                tile_data
            }
        };

        let rect = Rect::from_w_h(tile_data.rect.w() / 2.0, tile_data.rect.h() / 2.0);

        let (top_left, top_right, bottom_left, bottom_right) = (
            TileData::new(
                rect.top_left_of(tile_data.rect)
                    .pad_right(1.0)
                    .pad_bottom(1.0),
            ),
            TileData::new(
                rect.top_right_of(tile_data.rect)
                    .pad_left(1.0)
                    .pad_bottom(1.0),
            ),
            TileData::new(
                rect.bottom_left_of(tile_data.rect)
                    .pad_right(1.0)
                    .pad_top(1.0),
            ),
            TileData::new(
                rect.bottom_right_of(tile_data.rect)
                    .pad_left(1.0)
                    .pad_top(1.0),
            ),
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

pub fn create_tiling<T>(tile: T, steps: u32) -> Vec<T>
where
    T: Divide,
{
    let mut tiles = vec![tile];

    for _ in 0..steps {
        tiles = tiles.into_iter().flat_map(|t| t.divide()).collect();
    }

    tiles
}
