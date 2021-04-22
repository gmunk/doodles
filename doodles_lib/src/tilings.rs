use nannou::geom::Rect;

pub trait Divide {
    fn divide(&self) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum WandererTileOrientation {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
pub enum WandererTile {
    LeftHanded(TileData, WandererTileOrientation),
    RightHanded(TileData, WandererTileOrientation),
}

impl Divide for WandererTile {
    fn divide(&self) -> Vec<Self> {
        let rect = match self {
            WandererTile::LeftHanded(tile_data, _) | WandererTile::RightHanded(tile_data, _) => {
                Rect::from_w_h(tile_data.rect.w() / 2.0, tile_data.rect.h() / 2.0)
            }
        };

        match self {
            WandererTile::LeftHanded(tile_data, tile_orientation) => match tile_orientation {
                WandererTileOrientation::Bottom => {
                    vec![
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                    ]
                }
                WandererTileOrientation::Left => {
                    vec![
                        WandererTile::RightHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                    ]
                }
                WandererTileOrientation::Top => {
                    vec![
                        WandererTile::RightHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                    ]
                }
                WandererTileOrientation::Right => {
                    vec![
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                    ]
                }
            },
            WandererTile::RightHanded(tile_data, tile_orientation) => match tile_orientation {
                WandererTileOrientation::Bottom => {
                    vec![
                        WandererTile::RightHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                    ]
                }
                WandererTileOrientation::Left => {
                    vec![
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                    ]
                }
                WandererTileOrientation::Top => {
                    vec![
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Left,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                    ]
                }
                WandererTileOrientation::Right => {
                    vec![
                        WandererTile::RightHanded(
                            TileData::new(rect.top_left_of(tile_data.rect)),
                            WandererTileOrientation::Top,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.top_right_of(tile_data.rect)),
                            WandererTileOrientation::Bottom,
                        ),
                        WandererTile::RightHanded(
                            TileData::new(rect.bottom_left_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
                        WandererTile::LeftHanded(
                            TileData::new(rect.bottom_right_of(tile_data.rect)),
                            WandererTileOrientation::Right,
                        ),
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
