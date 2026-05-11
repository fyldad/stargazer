use crate::{CONSTELLATION_SIZE, HEIGHT, STAR_PALETTE, WIDTH};
use rand::RngExt;
use std::fmt::{Display, Formatter};
use get_enum_variant_trait::GetEnumVariant;
use crate::spectral_type::SpectralType;

#[derive(Debug)]
pub struct Star {
    pub(crate) width: u16,
    pub(crate) height: u16,
    magnitude: u8,
    pub spectral_type: SpectralType,
}

impl Display for Star {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let star = STAR_PALETTE[STAR_PALETTE.len() - 1 - self.magnitude as usize];
        write!(f, "{}", star)
    }
}

impl Star {
    pub(crate) fn generate_star() -> Self {
        let mut rng = rand::rng();

        Star {
            width: rng.random_range(0..WIDTH),
            height: rng.random_range(0..HEIGHT),
            magnitude: rng.random_range(0..STAR_PALETTE.len() as u8),
            spectral_type: SpectralType::get_enum_variant(rng),
        }
    }

    pub(crate) fn get_constellation_key(&self) -> String {
        let x_group = self.width / CONSTELLATION_SIZE;
        let y_group = self.height / CONSTELLATION_SIZE;
        format!("{}:{}", x_group, y_group)
    }
}


