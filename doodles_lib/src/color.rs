use nannou::color::{self, Srgb};

pub type Rgb = Srgb<u8>;

#[derive(Copy, Clone)]
pub enum Color {
    Skobeloff,
    ChampagnePink,
    InternationalOrangeGoldenGateBridge,
    EerieBlack,
    RedPigment,
    MintCream,
    SpaceCadet,
    Cerise,
}

impl Color {
    fn value(&self) -> (u8, u8, u8) {
        match self {
            Color::Skobeloff => (25u8, 114u8, 120u8),
            Color::ChampagnePink => (237u8, 221u8, 212u8),
            Color::InternationalOrangeGoldenGateBridge => (196u8, 69u8, 54u8),
            Color::EerieBlack => (37u8, 36u8, 34u8),
            Color::RedPigment => (255u8, 0u8, 34u8),
            Color::MintCream => (243u8, 247u8, 240u8),
            Color::SpaceCadet => (57u8, 47u8, 90u8),
            Color::Cerise => (218u8, 65u8, 103u8),
        }
    }
}

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.value();
        color::srgb(r, g, b)
    }
}
