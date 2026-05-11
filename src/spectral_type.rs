use rand::RngExt;
use get_enum_variant::GetEnumVariant;
use ratatui::prelude::Color;

#[derive(GetEnumVariant, Debug)]
pub(crate) enum SpectralType {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl SpectralType {
    pub(crate) fn get_color(&self) -> Color {
        match self {
            SpectralType::O => Color::Rgb(72, 184, 211),
            SpectralType::B => Color::Rgb(167, 218, 232),
            SpectralType::A => Color::Rgb(251, 254, 232),
            SpectralType::F => Color::Rgb(253, 250, 205),
            SpectralType::G => Color::Rgb(251, 229, 8),
            SpectralType::K => Color::Rgb(242, 131, 0),
            SpectralType::M => Color::Rgb(224, 64, 39),
        }
    }
}
