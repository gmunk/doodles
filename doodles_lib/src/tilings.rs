use nannou::geom::Rect;

pub struct TileData {
    pub rect: Rect,
}

impl TileData {
    fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

pub enum DominoTile {
    Horizontal(TileData),
    Vertical(TileData),
}

impl DominoTile {
    fn divide(&self) -> Vec<DominoTile> {
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

pub fn create_domino_tiling(rect: Rect, steps: u32) -> Vec<DominoTile> {
    let mut tiles = vec![DominoTile::Horizontal(TileData::new(rect))];

    for _ in 0..steps {
        tiles = tiles.into_iter().flat_map(|t| t.divide()).collect();
    }

    tiles
}
