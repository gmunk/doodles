use nannou::color::{self, Srgb};

pub type Rgb = Srgb<u8>;

#[derive(Copy, Clone)]
pub enum Color {
    Skobeloff,
    ChampagnePink,
    InternationalOrangeGoldenGateBridge,
    SagebrushGreen,
    Marsala,
    GraniteGray,
    Red032,
    Yellow47,
    Navy2380,
}

impl Color {
    fn value(&self) -> (u8, u8, u8) {
        match self {
            Color::Skobeloff => (25u8, 114u8, 120u8),
            Color::ChampagnePink => (237u8, 221u8, 212u8),
            Color::InternationalOrangeGoldenGateBridge => (196u8, 69u8, 54u8),
            Color::SagebrushGreen => (86u8, 117u8, 114u8),
            Color::Marsala => (150u8, 79u8, 76u8),
            Color::GraniteGray => (105u8, 102u8, 103u8),
            Color::Red032 => (246u8, 80u8, 88u8),
            Color::Yellow47 => (251u8, 222u8, 68u8),
            Color::Navy2380 => (40u8, 51u8, 74u8),
        }
    }
}

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.value();
        color::srgb(r, g, b)
    }
}
